// ═══════════════════════════════════════════════════════════════════════════════
// TRINITY ID AI OS — trinity-voice/src/audio.rs
// ═══════════════════════════════════════════════════════════════════════════════
//
// PURPOSE:     Audio I/O handling for voice interface
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::AudioChunk;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/// Audio input handler
pub struct AudioInput {
    pub sample_rate: u32,
    pub channels: u16,
    tx: mpsc::Sender<AudioChunk>,
    _stop_tx: Option<oneshot::Sender<()>>,
}

impl std::fmt::Debug for AudioInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioInput")
            .field("sample_rate", &self.sample_rate)
            .field("channels", &self.channels)
            .finish()
    }
}

impl AudioInput {
    pub fn new(sample_rate: u32, channels: u16) -> (Self, mpsc::Receiver<AudioChunk>) {
        let (tx, rx) = mpsc::channel(100);
        (
            Self {
                sample_rate,
                channels,
                tx,
                _stop_tx: None,
            },
            rx,
        )
    }

    /// Start capturing audio
    pub async fn start(&mut self) -> anyhow::Result<()> {
        let (stop_tx, mut stop_rx) = oneshot::channel();
        self._stop_tx = Some(stop_tx);

        let tx = self.tx.clone();
        let sample_rate = self.sample_rate;
        let channels = self.channels;

        std::thread::spawn(move || {
            let host = cpal::default_host();
            let device = match host.default_input_device() {
                Some(d) => d,
                None => {
                    tracing::error!("No input device available");
                    return;
                }
            };

            let config = cpal::StreamConfig {
                channels,
                sample_rate: cpal::SampleRate(sample_rate),
                buffer_size: cpal::BufferSize::Default,
            };

            let stream_result = device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let chunk = AudioChunk {
                        data: data.to_vec(),
                        timestamp_ms: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64,
                        sample_rate,
                    };
                    let _ = tx.try_send(chunk);
                },
                |err| tracing::error!("an error occurred on stream: {}", err),
                None,
            );

            match stream_result {
                Ok(stream) => {
                    if let Err(e) = stream.play() {
                        tracing::error!("failed to play stream: {}", e);
                        return;
                    }
                    // Wait for stop signal
                    let _ = stop_rx.try_recv(); // polling or blocking wait
                    while stop_rx.try_recv() == Err(oneshot::error::TryRecvError::Empty) {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                }
                Err(e) => {
                    tracing::error!("failed to build input stream: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Stop capturing
    pub async fn stop(&mut self) -> anyhow::Result<()> {
        if let Some(tx) = self._stop_tx.take() {
            let _ = tx.send(());
        }
        Ok(())
    }
}

/// Audio output handler
pub struct AudioOutput {
    pub sample_rate: u32,
    pub channels: u16,
    tx: mpsc::Sender<AudioChunk>,
}

impl std::fmt::Debug for AudioOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioOutput")
            .field("sample_rate", &self.sample_rate)
            .field("channels", &self.channels)
            .finish()
    }
}

impl AudioOutput {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        let (tx, mut rx) = mpsc::channel::<AudioChunk>(100);

        std::thread::spawn(move || {
            let (_stream, stream_handle) = match rodio::OutputStream::try_default() {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("failed to get default output stream: {}", e);
                    return;
                }
            };
            let sink = match rodio::Sink::try_new(&stream_handle) {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("failed to create rodio sink: {}", e);
                    return;
                }
            };

            while let Some(chunk) = rx.blocking_recv() {
                let buffer = rodio::buffer::SamplesBuffer::new(channels, sample_rate, chunk.data);
                sink.append(buffer);
                sink.sleep_until_end();
            }
        });

        Self {
            sample_rate,
            channels,
            tx,
        }
    }

    /// Start the output stream and sink
    pub fn initialize(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Play audio chunk
    pub async fn play(&self, chunk: &AudioChunk) -> anyhow::Result<()> {
        let _ = self.tx.send(chunk.clone()).await;
        Ok(())
    }

    /// Stop playback
    pub async fn stop(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

/// Audio preprocessing (noise reduction, normalization)
pub fn preprocess_audio(chunk: &mut AudioChunk) {
    // Basic normalization for now
    let max_val = chunk
        .data
        .iter()
        .fold(0.0f32, |max, &val| max.max(val.abs()));
    if max_val > 0.0 {
        for val in &mut chunk.data {
            *val /= max_val;
        }
    }
}
