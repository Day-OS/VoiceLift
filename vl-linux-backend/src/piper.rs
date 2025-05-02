use piper_rs::synth::{
    AudioOutputConfig, PiperSpeechStreamParallel,
    PiperSpeechSynthesizer,
};
use rodio::buffer::SamplesBuffer;
use std::path::Path;
use thiserror::Error;
const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Error, Debug)]
#[error("data store disconnected")]
pub enum VlPiperError {
    #[error("Piper Error")]
    Piper(#[from] piper_rs::PiperError),
    #[error("Rodio Play Error")]
    RodioPlayer(#[from] rodio::PlayError),
    #[error("Rodio Stream Error")]
    RodioStream(#[from] rodio::StreamError),
}

pub struct PiperTTSManager {
    model: PiperSpeechSynthesizer,
    rodio_sink: rodio::Sink,
    _rodio_stream: rodio::OutputStream,
    _rodio_handle: rodio::OutputStreamHandle,
}
unsafe impl Sync for PiperTTSManager {}
unsafe impl Send for PiperTTSManager {}

impl PiperTTSManager {
    pub fn new(
        model_config_path: &Path,
        speaker_id: i64,
    ) -> Result<Self, VlPiperError> {
        let model = piper_rs::from_config_path(model_config_path)?;
        model.set_speaker(speaker_id);
        let speech_synthesizer = PiperSpeechSynthesizer::new(model)?;

        let (rodio_stream, rodio_handle) =
            rodio::OutputStream::try_default()?;
        let rodio_sink = rodio::Sink::try_new(&rodio_handle)?;

        let manager = Self {
            model: speech_synthesizer,
            rodio_sink,
            _rodio_stream: rodio_stream,
            _rodio_handle: rodio_handle,
        };

        Ok(manager)
    }

    pub fn speak(
        &self,
        text: String,
        pitch: u8,
        volume: u8,
    ) -> Result<(), VlPiperError> {
        let audio = self.model.synthesize_parallel(
            text.clone(),
            Some(AudioOutputConfig {
                volume: Some(volume),
                pitch: Some(pitch),
                appended_silence_ms: None,
                rate: None,
            }),
        )?;
        let buf = stream_to_sample_buffer(audio)?;

        self.rodio_sink.append(buf);
        log::info!("Playing audio... {}", text);
        self.rodio_sink.sleep_until_end();
        Ok(())
    }

    pub fn get_handle_name() -> String {
        format!("alsa_playback.{}", CARGO_PKG_NAME)
    }
}

fn stream_to_sample_buffer(
    stream: PiperSpeechStreamParallel,
) -> Result<SamplesBuffer<f32>, VlPiperError> {
    // Converts Audio into Vec<f32> samples
    let mut samples: Vec<f32> = Vec::new();
    let mut samplerate: u32 = 22050;
    for audio in stream {
        let _audio = audio?;
        samplerate = _audio.info.sample_rate as u32;
        samples.append(&mut _audio.into_vec());
    }
    let buf = SamplesBuffer::new(1, samplerate, samples);
    Ok(buf)
}
