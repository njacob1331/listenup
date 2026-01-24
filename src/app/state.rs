use gpui::Global;

use crate::audio::{engine::AudioEngine, recording::Recording};

pub struct State {
    pub audio_engine: AudioEngine,
    pub recording: Option<Recording>,
}

impl Global for State {}

impl State {
    pub fn new() -> Self {
        Self {
            audio_engine: AudioEngine::new(),
            recording: None,
        }
    }

    pub fn store_recording(&mut self, recording: Recording) {
        self.recording = Some(recording)
    }

    pub fn audio_engine(&self) -> &AudioEngine {
        &self.audio_engine
    }
}
