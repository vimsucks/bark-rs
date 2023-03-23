use crate::device;
use confy;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub devices: Vec<device::Device>,
}

pub fn load() -> Config {
    println!(
        "{}",
        confy::get_configuration_file_path("bark", None)
            .unwrap()
            .display()
    );
    confy::load("bark", "config").expect("load config failed")
}

pub fn save(cfg: &Config) {
    confy::store("bark", "config", cfg).expect("store config failed")
}
