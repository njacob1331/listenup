use cpal::traits::DeviceTrait;
use cpal::{Device, SupportedStreamConfig};

#[derive(Debug)]
pub struct DeviceSpecs {
    id: String,
    name: String,
    config: Option<SupportedStreamConfig>,
}

impl From<&Device> for DeviceSpecs {
    fn from(device: &Device) -> Self {
        let id = device
            .id()
            .map_or_else(|_err| "UNKNOWN".to_string(), |id| id.1);
        let name = device.description().map_or_else(
            |_err| "Unknown device".to_string(),
            |desc| desc.name().to_string(),
        );

        let config = if device.supports_input() {
            device.default_input_config().ok()
        } else {
            device.default_output_config().ok()
        };

        Self { id, name, config }
    }
}

impl DeviceSpecs {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn config(&self) -> Option<&SupportedStreamConfig> {
        self.config.as_ref()
    }
}
