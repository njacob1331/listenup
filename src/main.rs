use std::{sync::Arc, time::Duration};

use crate::{
    app::run_app,
    audio::{engine::AudioEngine, sample::Sample},
};

mod app;
mod audio;

fn main() {
    // let mut audio_engine = AudioEngine::new();
    // let recording = audio_engine.record();

    // std::thread::sleep(Duration::from_secs(100));

    // audio_engine.stop_recording();

    // let sample: Arc<Sample> = Arc::new(recording.into());
    // println!("num samples: {:?}", sample.num_samples());

    run_app();
}
