use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::{Device, Devices, Host, HostId};
use std::fmt;

pub struct AudioDevices {
    pub(crate) host: Host,
}

impl AudioDevices {
    pub fn new() -> Self {
        Self {
            host: cpal::default_host(),
        }
    }

    pub fn host_id(&self) -> HostId {
        self.host.id()
    }
    pub fn default_input_device(&self) -> Device {
        self.host
            .default_input_device()
            .expect("no input device available")
    }

    pub fn default_output_device(&self) -> Device {
        self.host
            .default_output_device()
            .expect("no output device available")
    }

    pub fn devices(&self) -> Devices {
        self.host.devices().expect("no devices")
    }

    // pub fn output_devices(&self) -> Devices {
    //     self.host.output_devices().expect("no output devices")
    // }
    //
    // pub fn input_devices(&self) -> Devices {
    //     self.host.input_devices().expect("no input devices")
    // }
}

impl fmt::Display for AudioDevices {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "
=====================================
| Default host: {:?}
| Default input device: {:?}
| Default output device: {:?}
=====================================
Devices:
",
                self.host_id(),
                self.default_input_device().name().unwrap(),
                self.default_output_device().name().unwrap(),
            )
        )?;
        for (device_index, device) in self.devices().enumerate() {
            let name = device.name().unwrap();
            if let Ok(conf) = device.default_input_config() {
                println!("    Default input stream config:\n      {:?}", conf);
            }
            let input_configs = match device.supported_input_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    println!(
                        "    Error getting supported input configs: {:?}",
                        e
                    );
                    Vec::new()
                }
            };
            if !input_configs.is_empty() {
                println!("    All supported input stream configs:");
                for (config_index, config) in
                    input_configs.into_iter().enumerate()
                {
                    println!(
                        "      {}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    );
                }
            }

            // Output configs
            if let Ok(conf) = device.default_output_config() {
                println!("    Default output stream config:\n      {:?}", conf);
            }
            let output_configs = match device.supported_output_configs() {
                Ok(f) => f.collect(),
                Err(e) => {
                    println!(
                        "    Error getting supported output configs: {:?}",
                        e
                    );
                    Vec::new()
                }
            };
            if !output_configs.is_empty() {
                println!("    All supported output stream configs:");
                for (config_index, config) in
                    output_configs.into_iter().enumerate()
                {
                    println!(
                        "      {}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    );
                }
            }
            write!(f, "\t-{}\n", device.name().unwrap())?;
        }

        Ok(())
    }
}
