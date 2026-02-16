use crate::error::VoiceError;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat, Stream};
use crossbeam_channel::Sender;

/// Start capturing audio from the default input device.
///
/// # Arguments
/// * `tx` - Channel sender for captured audio frames (f32 PCM samples)
///
/// # Returns
/// * `Ok(Stream)` - Active audio stream (must be kept alive)
/// * `Err(VoiceError)` - If no input device available or stream creation fails
///
/// # Notes
/// - Uses device's default sample rate and channel count (will document resampling need)
/// - Audio callback uses `try_send` to avoid blocking real-time thread
/// - Caller must keep returned Stream alive to continue capture
pub fn start_capture(tx: Sender<Vec<f32>>) -> Result<Stream, VoiceError> {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .ok_or_else(|| VoiceError::AudioDeviceNotFound("No default input device found".to_string()))?;

    let config = device
        .default_input_config()
        .map_err(|e| VoiceError::AudioStreamFailed(format!("Failed to get input config: {}", e)))?;

    // Note: Using device's default config for now
    // TODO: Future resampling to 48kHz mono for Opus VoIP if device differs
    let sample_rate = config.sample_rate();
    let channels = config.channels();

    eprintln!(
        "Voice capture starting: {}Hz, {} channel(s), format: {:?}",
        sample_rate, channels, config.sample_format()
    );

    let stream = match config.sample_format() {
        SampleFormat::F32 => build_input_stream::<f32>(&device, &config.into(), tx)?,
        SampleFormat::I16 => build_input_stream::<i16>(&device, &config.into(), tx)?,
        SampleFormat::U16 => build_input_stream::<u16>(&device, &config.into(), tx)?,
        _ => {
            return Err(VoiceError::AudioStreamFailed(
                format!("Unsupported sample format: {:?}", config.sample_format())
            ))
        }
    };

    stream
        .play()
        .map_err(|e| VoiceError::AudioStreamFailed(format!("Failed to start stream: {}", e)))?;

    Ok(stream)
}

// Specialized implementations for each sample format
fn build_input_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    tx: Sender<Vec<f32>>,
) -> Result<Stream, VoiceError>
where
    T: cpal::Sample + cpal::SizedSample,
    f32: cpal::FromSample<T>,
{
    let tx = tx.clone();

    let stream = device
        .build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                // Convert samples to f32 using FromSample trait
                let samples: Vec<f32> = data.iter().map(|&s| f32::from_sample(s)).collect();

                // Use try_send to avoid blocking audio thread
                // Dropped frames are acceptable under load (real-time constraint)
                let _ = tx.try_send(samples);
            },
            move |err| {
                eprintln!("Audio capture error: {}", err);
            },
            None,
        )
        .map_err(|e| VoiceError::AudioStreamFailed(format!("Failed to build input stream: {}", e)))?;

    Ok(stream)
}
