use std::fs::File;

use interception::{Device, ScanCode};
use serde::{Deserialize, Serialize};

use crate::keydbkey_serde::ScanCodeDef;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub keys: KeysConfig,

    #[serde(rename = "keyboardId")]
    pub keyboard_id: Device,

    pub wait: u64
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keys: KeysConfig {
                up: ScanCode::W,
                left: ScanCode::A,
                down: ScanCode::S,
                right: ScanCode::D,
                stratagem: ScanCode::LeftControl
            },
            keyboard_id: 1,
            wait: 30
        }
    }
}

impl Config {
    pub fn load() -> crate::Config {
        match File::open("data/config.yml") {
            Ok(file) => serde_yaml::from_reader(&file).or_else(|_| {
                println!("Unable to parse config file");
                Ok::<crate::Config, Box<dyn std::error::Error>>(crate::Config::default())
            }).unwrap(),
            _ => {
                println!("No config file");
                crate::Config::default()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeysConfig {
    #[serde(with = "ScanCodeDef")]
    pub up: ScanCode,
    #[serde(with = "ScanCodeDef")]
    pub down: ScanCode,
    #[serde(with = "ScanCodeDef")]
    pub left: ScanCode,
    #[serde(with = "ScanCodeDef")]
    pub right: ScanCode,
    #[serde(with = "ScanCodeDef")]
    pub stratagem: ScanCode,
}