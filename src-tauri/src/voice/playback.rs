use crate::error::VoiceError;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, Stream};
use crossbeam_channel::Receiver;

/// Start audio playback on the default output device
///
/// # Arguments
/// * `rx` - Channel receiver for mixed audio frames (f32 PCM samples)
///
/// # Returns
/// * `Ok(Stream)` - Active audio stream (must be kept alive)
/// * `Err(VoiceError)` - If no output device available or stream creation fails
///
/// # Notes
/// - Uses device's default sample rate and channel count
/// - Audio callback uses `try_recv` to avoid blocking real-time thread
/// - Outputs silence when no data available (graceful underrun handling)
/// - Caller must keep returned Stream alive to continue playback
pub fn start_playback(rx: Receiver<Vec<f32>>) -> Result<Stream, VoiceError> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| VoiceError::AudioDeviceNotFound("No default output device found".to_string()))?;

    let config = device
        .default_output_config()
        .map_err(|e| VoiceError::AudioStreamFailed(format!("Failed to get output config: {}", e)))?;

    // Note: Using device's default config for now
    let sample_rate = config.sample_rate();
    let channels = config.channels();

    eprintln!(
        "Voice playback starting: {}Hz, {} channel(s), format: {:?}",
        sample_rate, channels, config.sample_format()
    );

    let stream = match config.sample_format() {
        SampleFormat::F32 => build_output_stream::<f32>(&device, &config.into(), rx)?,
        SampleFormat::I16 => build_output_stream::<i16>(&device, &config.into(), rx)?,
        SampleFormat::U16 => build_output_stream::<u16>(&device, &config.into(), rx)?,
        _ => {
            return Err(VoiceError::AudioStreamFailed(
                format!("Unsupported sample format: {:?}", config.sample_format())
            ))
        }
    };

    stream
        .play()
        .map_err(|e| VoiceError::AudioStreamFailed(format!("Failed to start playback: {}", e)))?;

    Ok(stream)
}

fn build_output_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    rx: Receiver<Vec<f32>>,
) -> Result<Stream, VoiceError>
where
    T: cpal::Sample + cpal::SizedSample,
    f32: cpal::FromSample<T>,
    T: FromSample<f32>,
{
    let rx = rx.clone();
    let mut buffer = Vec::new();
    let mut buffer_pos = 0;

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for sample_out in data.iter_mut() {
                    // If we've consumed the current buffer, try to get a new one
                    if buffer_pos >= buffer.len() {
                        match rx.try_recv() {
                            Ok(new_buffer) => {
                                buffer = new_buffer;
                                buffer_pos = 0;
                            }
                            Err(_) => {
                                // No data available - output silence
                                buffer.clear();
                                buffer_pos = 0;
                            }
                        }
                    }

                    // Output sample (or silence if no data)
                    let sample = if buffer_pos < buffer.len() {
                        let val = buffer[buffer_pos];
                        buffer_pos += 1;
                        val
                    } else {
                        0.0
                    };

                    *sample_out = T::from_sample(sample);
                }
            },
            move |err| {
                eprintln!("Audio playback error: {}", err);
            },
            None,
        )
        .map_err(|e| VoiceError::AudioStreamFailed(format!("Failed to build output stream: {}", e)))?;

    Ok(stream)
}
