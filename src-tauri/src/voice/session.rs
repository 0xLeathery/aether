use libp2p::PeerId;
use libp2p_stream as stream;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;
use cpal::Stream as CpalStream;
use crossbeam_channel::bounded;
use futures::StreamExt;

use crate::error::VoiceError;
use super::{
    AudioFrame, AudioMixer, VoiceEncoder, VoiceDecoder,
    FRAME_SIZE, start_capture, start_playback,
    protocol::{VoicePacket, VOICE_PROTOCOL, send_frame, recv_frame},
};

/// Voice session manager
///
/// Coordinates the complete voice pipeline:
/// capture (cpal) → encode (opus) → send (libp2p stream) → [network] →
/// receive → decode → jitter buffer → mix → playback (cpal)
pub struct VoiceSession {
    is_active: Arc<AtomicBool>,
    participants: Arc<RwLock<HashSet<PeerId>>>,
    capture_stream: Option<CpalStream>,
    playback_stream: Option<CpalStream>,
    mixer: Arc<RwLock<AudioMixer>>,
    sequence: Arc<AtomicU32>,
    max_participants: usize,
}

impl VoiceSession {
    /// Create a new voice session (not started)
    pub fn new() -> Self {
        Self {
            is_active: Arc::new(AtomicBool::new(false)),
            participants: Arc::new(RwLock::new(HashSet::new())),
            capture_stream: None,
            playback_stream: None,
            mixer: Arc::new(RwLock::new(AudioMixer::new())),
            sequence: Arc::new(AtomicU32::new(0)),
            max_participants: 8,
        }
    }

    /// Join a voice session with specified peers
    ///
    /// Starts audio capture, encoding, streaming to peers, receiving from peers,
    /// decoding, mixing, and playback.
    ///
    /// Enforces 8-participant limit (7 peers + self).
    pub async fn join(
        &mut self,
        app: AppHandle,
        stream_control: stream::Control,
        peer_ids: Vec<PeerId>,
    ) -> Result<(), VoiceError> {
        // Check not already in session
        if self.is_active.load(Ordering::Relaxed) {
            return Err(VoiceError::AlreadyInSession);
        }

        // Check participant limit (7 peers + self = 8 max)
        if peer_ids.len() > 7 {
            return Err(VoiceError::SessionFull);
        }

        // Activate session
        self.is_active.store(true, Ordering::Relaxed);

        // Add all peers to participants
        {
            let mut participants = self.participants.write().await;
            for peer_id in peer_ids.iter() {
                participants.insert(*peer_id);
            }
        }

        // Initialize mixer with all peers
        {
            let mut mixer = self.mixer.write().await;
            for peer_id in peer_ids.iter() {
                mixer.add_peer(*peer_id)?;
            }
        }

        // Create channels for audio pipeline
        // Bounded with capacity 10 = 200ms of 20ms frames
        let (capture_tx, capture_rx) = bounded::<Vec<f32>>(10);
        let (playback_tx, playback_rx) = bounded::<Vec<f32>>(10);

        // Start audio capture
        let capture_stream = start_capture(capture_tx)?;
        self.capture_stream = Some(capture_stream);

        // Start audio playback
        let playback_stream = start_playback(playback_rx)?;
        self.playback_stream = Some(playback_stream);

        // Spawn encode-and-send task
        {
            let is_active = Arc::clone(&self.is_active);
            let participants = Arc::clone(&self.participants);
            let sequence = Arc::clone(&self.sequence);
            let mut stream_control = stream_control.clone();

            tokio::spawn(async move {
                let mut encoder = match VoiceEncoder::new() {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("Failed to create encoder: {}", e);
                        return;
                    }
                };

                // Persistent stream cache per peer
                let mut streams: HashMap<PeerId, libp2p::Stream> = HashMap::new();

                while is_active.load(Ordering::Relaxed) {
                    // Receive PCM from capture (blocking is OK in this tokio task)
                    let pcm = match capture_rx.recv() {
                        Ok(p) => p,
                        Err(_) => break, // Channel closed
                    };

                    // Encode to Opus
                    let opus_data = match encoder.encode(&pcm) {
                        Ok(data) => data,
                        Err(e) => {
                            eprintln!("Encode error: {}", e);
                            continue;
                        }
                    };

                    // Increment sequence number
                    let seq = sequence.fetch_add(1, Ordering::Relaxed);
                    let packet = VoicePacket::new(seq, opus_data);

                    // Send to all participants
                    let peer_list = participants.read().await.clone();
                    for peer_id in peer_list {
                        // Try to get or create stream for this peer
                        let stream = match streams.get_mut(&peer_id) {
                            Some(s) => s,
                            None => {
                                // Open new stream to peer
                                match stream_control.open_stream(peer_id, VOICE_PROTOCOL).await {
                                    Ok(s) => {
                                        streams.insert(peer_id, s);
                                        streams.get_mut(&peer_id).unwrap()
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to open stream to {}: {}", peer_id, e);
                                        continue;
                                    }
                                }
                            }
                        };

                        // Send frame
                        if let Err(e) = send_frame(stream, &packet).await {
                            eprintln!("Failed to send to {}: {}", peer_id, e);
                            // Remove broken stream from cache
                            streams.remove(&peer_id);
                        }
                    }
                }
            });
        }

        // Spawn receive-and-decode task
        {
            let is_active = Arc::clone(&self.is_active);
            let mixer = Arc::clone(&self.mixer);
            let mut stream_control = stream_control.clone();

            tokio::spawn(async move {
                while is_active.load(Ordering::Relaxed) {
                    // Accept incoming voice streams
                    let mut incoming = stream_control.accept(VOICE_PROTOCOL).unwrap();

                    while is_active.load(Ordering::Relaxed) {
                        match incoming.next().await {
                            Some((peer_id, mut stream)) => {
                                // Spawn per-peer decode task
                                let is_active = Arc::clone(&is_active);
                                let mixer = Arc::clone(&mixer);

                                tokio::spawn(async move {
                                    let mut decoder = match VoiceDecoder::new() {
                                        Ok(d) => d,
                                        Err(e) => {
                                            eprintln!("Failed to create decoder for {}: {}", peer_id, e);
                                            return;
                                        }
                                    };

                                    while is_active.load(Ordering::Relaxed) {
                                        // Receive frame from peer
                                        let packet = match recv_frame(&mut stream).await {
                                            Ok(p) => p,
                                            Err(e) => {
                                                eprintln!("Stream closed for {}: {}", peer_id, e);
                                                break;
                                            }
                                        };

                                        // Decode Opus to PCM
                                        let pcm = match decoder.decode(&packet.opus_data) {
                                            Ok(p) => p,
                                            Err(e) => {
                                                eprintln!("Decode error from {}: {}", peer_id, e);
                                                // Use PLC for this frame
                                                match decoder.decode_plc() {
                                                    Ok(p) => p,
                                                    Err(_) => continue,
                                                }
                                            }
                                        };

                                        // Feed to mixer's jitter buffer
                                        let frame = AudioFrame {
                                            sequence: packet.sequence,
                                            data: pcm,
                                            received_at: Instant::now(),
                                        };

                                        mixer.write().await.feed_frame(&peer_id, frame);
                                    }
                                });
                            }
                            None => break, // No more incoming streams
                        }
                    }
                }
            });
        }

        // Spawn mix-to-playback task
        {
            let is_active = Arc::clone(&self.is_active);
            let mixer = Arc::clone(&self.mixer);

            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_millis(20));

                while is_active.load(Ordering::Relaxed) {
                    interval.tick().await;

                    // Mix next frame
                    let mixed_audio = mixer.write().await.mix_next_frame(FRAME_SIZE);

                    // Send to playback (non-blocking)
                    if playback_tx.try_send(mixed_audio).is_err() {
                        // Playback buffer full, drop frame (acceptable)
                    }
                }
            });
        }

        // Emit session joined event
        let participant_ids: Vec<String> = {
            self.participants.read().await
                .iter()
                .map(|p| p.to_string())
                .collect()
        };

        let _ = app.emit("voice-session-joined", participant_ids);

        Ok(())
    }

    /// Leave the voice session
    ///
    /// Stops all audio processing and cleans up resources
    pub async fn leave(&mut self, app: AppHandle) {
        // Deactivate session (stops all spawned tasks)
        self.is_active.store(false, Ordering::Relaxed);

        // Drop audio streams (stops cpal)
        self.capture_stream = None;
        self.playback_stream = None;

        // Clear mixer
        {
            let mut mixer = self.mixer.write().await;
            let peer_ids: Vec<PeerId> = self.participants.read().await.iter().copied().collect();
            for peer_id in peer_ids {
                mixer.remove_peer(&peer_id);
            }
        }

        // Clear participants
        self.participants.write().await.clear();

        // Reset sequence
        self.sequence.store(0, Ordering::Relaxed);

        // Emit session left event
        let _ = app.emit("voice-session-left", ());
    }

    /// Get current participant count
    pub fn participant_count(&self) -> usize {
        if let Ok(participants) = self.participants.try_read() {
            participants.len()
        } else {
            0
        }
    }

    /// Check if currently in a voice session
    pub fn is_in_session(&self) -> bool {
        self.is_active.load(Ordering::Relaxed)
    }
}
