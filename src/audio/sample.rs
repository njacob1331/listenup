use std::sync::atomic::{AtomicUsize, Ordering};

use cpal::StreamConfig;

use crate::audio::recording::Recording;

#[derive(Debug)]
pub struct Sample {
    samples: Vec<f32>,
    config: StreamConfig,
    playback_index: AtomicUsize,
    playback_range: (usize, usize),
    should_loop: bool,
}

impl From<Recording> for Sample {
    fn from(recording: Recording) -> Self {
        let samples = recording.handle.join().expect("recording thread panicked");
        let num_samples = samples.len();

        Self {
            samples,
            config: recording.config,
            playback_index: AtomicUsize::new(0),
            playback_range: (0, num_samples),
            should_loop: true,
        }
    }
}

impl Sample {
    pub fn num_samples(&self) -> usize {
        self.samples.len()
    }

    pub fn samples(&self) -> &[f32] {
        &self.samples
    }

    pub fn config(&self) -> &StreamConfig {
        &self.config
    }

    pub fn playback_index(&self) -> usize {
        self.playback_index.load(Ordering::Relaxed)
    }

    pub fn iter_playback_index(&self) -> usize {
        let current = self.playback_index.load(Ordering::Relaxed);

        if current >= self.playback_range.1 {
            if self.should_loop {
                self.playback_index
                    .store(self.playback_range.0, Ordering::Relaxed);
                return self.playback_range.0;
            } else {
                return current;
            }
        }

        self.playback_index.fetch_add(1, Ordering::Relaxed)
    }

    pub fn set_playback_index(&self, index: usize) {
        self.playback_index.store(index, Ordering::Relaxed);
    }

    pub fn increment_playback_index(&self) {
        let _ = self.playback_index.fetch_add(1, Ordering::Relaxed);
    }

    pub fn set_playback_range(&mut self, start: usize, end: usize) {
        self.playback_range = (start, end)
    }

    pub fn should_loop(&self) -> bool {
        self.should_loop
    }
}
