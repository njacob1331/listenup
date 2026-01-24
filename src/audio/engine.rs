use std::{
    sync::{Arc, atomic::Ordering},
    time::Duration,
};

use crate::audio::{device::DeviceSpecs, recording::Recording, sample::Sample};
use cpal::{
    Device as CpalDevice, DevicesError, Host, Stream, StreamConfig, SupportedStreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use ringbuf::{
    HeapRb, SharedRb,
    storage::Heap,
    traits::{Consumer, Producer, Split},
    wrap::caching::Caching,
};
use tokio::sync::broadcast;

type StreamProducer = Caching<ringbuf::Arc<SharedRb<Heap<f32>>>, true, false>;
pub type StreamConsumer = Caching<ringbuf::Arc<SharedRb<Heap<f32>>>, false, true>;

#[derive(Debug, Clone, Copy)]
pub enum EngineState {
    Idle,
    Playback,
    Paused,
    Recording,
    Busy,
}

pub struct AudioEngine {
    host: Host,
    input_device: Option<CpalDevice>,
    output_device: Option<CpalDevice>,
    input_stream: Option<Stream>,
    output_stream: Option<Stream>,
    state: EngineState,
    state_tx: broadcast::Sender<EngineState>,
}

impl AudioEngine {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let input_device = host.default_input_device();
        let output_device = host.default_output_device();
        let (state_tx, _) = broadcast::channel::<EngineState>(16);

        Self {
            host,
            input_device,
            output_device,
            input_stream: None,
            output_stream: None,
            state: EngineState::Idle,
            state_tx,
        }
    }

    pub fn state(&self) -> EngineState {
        self.state
    }

    pub fn new_state_subscriber(&self) -> broadcast::Receiver<EngineState> {
        self.state_tx.subscribe()
    }

    pub fn set_state(&mut self, state: EngineState) {
        if let Err(e) = self.state_tx.send(state) {
            eprintln!("error reporting state: {e}")
        }

        self.state = state
    }

    pub fn input_device(&self) -> Option<DeviceSpecs> {
        self.input_device.as_ref().map(DeviceSpecs::from)
    }

    pub fn output_device(&self) -> Option<DeviceSpecs> {
        self.output_device.as_ref().map(DeviceSpecs::from)
    }

    pub fn input_devices(&self) -> Vec<DeviceSpecs> {
        self.host
            .input_devices()
            .expect("failed to get input devices")
            .map(|ref device| DeviceSpecs::from(device))
            .collect()
    }

    pub fn set_input_device(&mut self, device_id: &str) -> Result<(), DevicesError> {
        let mut input_devices = self.host.input_devices().unwrap();
        let new_device = input_devices.find(|device| device.id().unwrap().1 == device_id);

        if new_device.is_some() {
            self.input_device = new_device
        }

        Ok(())
    }

    fn output_devices(&self) -> Result<Vec<DeviceSpecs>, DevicesError> {
        Ok(self
            .host
            .output_devices()?
            .map(|ref device| DeviceSpecs::from(device))
            .collect())
    }

    fn set_output_device(&mut self, device_id: &str) -> Result<(), DevicesError> {
        let mut output_devices = self.host.output_devices().unwrap();
        let new_device = output_devices.find(|device| device.id().unwrap().1 == device_id);

        if let Some(new_device) = new_device {
            self.output_device = Some(new_device)
        }

        Ok(())
    }

    pub fn record(&mut self) -> Recording {
        self.state = EngineState::Recording;

        let device = self
            .input_device
            .as_ref()
            .expect("failed to get input device");
        let config: StreamConfig = device
            .default_input_config()
            .expect("failed to get config")
            .into();
        // let ring = HeapRb::<f32>::new((config.sample_rate * config.channels as u32 * 2) as usize);
        // let (mut producer, consumer) = ring.split();

        let (producer, consumer) = crossbeam_channel::bounded::<Vec<f32>>(8);

        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // for &sample in data {
            //     if let Err(e) = producer.try_send(sample) {
            //         eprintln!("sample dropped: {e}")
            //     }
            // }
            //
            let mut buf = Vec::with_capacity(data.len());
            buf.extend_from_slice(data);

            let _send = producer.try_send(buf);
        };

        let input_stream = device
            .build_input_stream(&config, input_data_fn, |err| eprintln!("{err}"), None)
            .expect("failed to build stream");
        input_stream.play().expect("failed to start input stream");
        self.input_stream = Some(input_stream);

        Recording::new(consumer, self.new_state_subscriber(), config)
    }

    pub fn stop_recording(&mut self) {
        self.input_stream = None;
        self.set_state(EngineState::Idle);
    }

    pub fn playback(&mut self, sample: Arc<Sample>) {
        self.state = EngineState::Playback;

        let output_device = self.output_device.as_ref().unwrap();
        let config: StreamConfig = output_device.default_output_config().unwrap().into();

        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let samples = sample.samples();
            let num_samples = samples.len();

            for s in data.iter_mut() {
                let playback_index = sample.playback_index();
                if playback_index < num_samples {
                    *s = samples[playback_index];
                    sample.set_playback_index(playback_index + 1);
                } else {
                    *s = 0.0;
                }
            }
        };

        let output_stream = output_device
            .build_output_stream(&config, output_data_fn, |err| eprintln!("{err}"), None)
            .expect("failed to build stream");
        output_stream.play().expect("failed to play output stream");
        self.output_stream = Some(output_stream);
    }

    pub fn pause(&mut self) {
        self.output_stream = None;
        self.set_state(EngineState::Paused);
    }
}
