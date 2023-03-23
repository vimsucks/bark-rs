use crate::conf;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub server_url: String,
    pub device_key: String,
    pub default: bool,
}

pub fn add(args: &crate::cli::AddDeviceArgs) {
    let device = Device {
        name: String::from(&args.name),
        server_url: String::from(&args.server_url),
        device_key: String::from(&args.device_key),
        default: args.default,
    };
    let mut cfg = conf::load();
    if args.default {
        for device in &mut cfg.devices {
            device.default = false
        }
    }
    cfg.devices.push(device);
    conf::save(&cfg);
    println!("device {} saved", &args.name)
}

pub fn list() {
    let cfg = conf::load();
    println!("{:?}", cfg)
}

pub fn default_device<'a>() -> Option<Device> {
    let cfg = conf::load();
    let mut iter = cfg.devices.iter();
    let index = iter.position(|device| device.default)?;
    cfg.devices.into_iter().nth(index)
}

pub fn find_device<'a>(device_name: &str) -> Option<Device> {
    let cfg = conf::load();
    let mut iter = cfg.devices.iter();
    let index = iter.position(|device| device.name.eq(device_name))?;
    cfg.devices.into_iter().nth(index)
}
