use crate::game::*;
use anyhow::{Error, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
    iwads: Vec<Iwad>,
    wad_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            iwads: Vec::new(),
            wad_dir: PathBuf::new(),
        }
    }
}

impl Config {
    pub fn load_config(file: &str) -> Self {
        toml::from_str(file).unwrap()
    }

    pub fn save_config(&self) -> Result<()> {
        let project_dir = ProjectDirs::from("", "", "rdl")
            .ok_or_else(|| Error::msg("Failed to get project dir"))?;
        let config = toml::to_string(self)?;
        let mut file = File::create(project_dir.config_dir().join("config.toml"))?;
        file.write_all(config.as_bytes()).map_err(Error::msg)
    }

    pub fn set_wad_dir(self, wad_dir: PathBuf) -> Self {
        Self { wad_dir, ..self }
    }

    pub fn add_iwad(self, iwad: Iwad) -> Self {
        let mut iwads = self.iwads.clone();
        iwads.push(iwad);
        Self { iwads, ..self }
    }
}
