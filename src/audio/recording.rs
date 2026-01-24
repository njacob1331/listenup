use std::{
    fs::File,
    io::Write,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    thread::JoinHandle,
};

use cpal::StreamConfig;
use tokio::sync::broadcast;

use crate::audio::engine::{EngineState, StreamConsumer};

use ringbuf::traits::Consumer;

#[derive(Debug)]
pub struct Recording {
    pub handle: JoinHandle<Vec<f32>>,
    pub config: StreamConfig,
    pub len: Arc<AtomicUsize>,
}

impl Recording {
    pub fn new(
        stream_consumer: crossbeam_channel::Receiver<Vec<f32>>,
        mut audio_engine_state: broadcast::Receiver<EngineState>,
        config: StreamConfig,
    ) -> Self {
        let ten_seconds = (config.sample_rate as usize * config.channels as usize) * 10;
        let recording_len = Arc::new(AtomicUsize::new(0));
        let recording_len_reporter = recording_len.clone();

        let handle = std::thread::spawn(move || {
            let mut samples = Vec::with_capacity(ten_seconds);

            loop {
                while let Ok(buffer) = stream_consumer.recv() {
                    samples.extend_from_slice(&buffer);
                    recording_len_reporter.fetch_add(buffer.len(), Ordering::Relaxed);
                }

                match audio_engine_state.try_recv() {
                    Ok(EngineState::Idle) => break,
                    _ => continue,
                }
            }

            samples
        });

        Self {
            handle,
            config,
            len: recording_len,
        }
    }

    fn write_to_disk(samples: &mut Vec<f32>) {
        let mut disk = File::create("recording.bin").unwrap();
        for sample in samples.drain(..) {
            println!("writing to disk: {sample}");
            disk.write_all(&sample.to_le_bytes())
                .expect("failed to write sample")
        }
    }
}
