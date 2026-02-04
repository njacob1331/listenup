use gpui::{App, AppContext, Context, Entity, EventEmitter};

use crate::core::{AudioEngine, EngineState, Recording};

pub trait Model
where
    Self: Sized,
{
    fn init(cx: &mut App) -> Entity<Self>;
    fn notify(cx: &mut Context<Self>);
}

pub struct AudioManager {
    audio_engine: AudioEngine,
    recording: Option<Recording>,
    samples: Vec<String>,
}

impl AudioManager {
    fn new() -> Self {
        Self {
            audio_engine: AudioEngine::new(),
            recording: None,
            samples: vec![],
        }
    }

    pub fn engine_state(&self) -> EngineState {
        self.audio_engine.state()
    }

    fn record(&mut self, cx: &mut Context<Self>) {
        let recording = self.audio_engine.record();
        let _old = self.recording.replace(recording);

        Self::notify(cx);
    }

    fn stop_recording(&mut self, cx: &mut Context<Self>) {
        self.audio_engine.stop_recording();
        let _sample = self.recording.take().unwrap();

        let mock_id = format!("sample {}", self.samples.len());
        self.samples.push(mock_id);

        Self::notify(cx);
    }

    pub fn toggle_recording(&mut self, cx: &mut Context<Self>) {
        match self.engine_state() {
            EngineState::Idle => self.record(cx),
            EngineState::Recording => self.stop_recording(cx),
            _ => {}
        }

        Self::notify(cx);
    }

    pub fn recording(&self) -> Option<&Recording> {
        self.recording.as_ref()
    }

    pub fn samples(&self) -> &[String] {
        &self.samples
    }
}

impl Model for AudioManager {
    fn init(cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| AudioManager::new())
    }

    fn notify(cx: &mut Context<Self>) {
        cx.notify();
    }
}
