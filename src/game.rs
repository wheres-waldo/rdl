use anyhow::{Error, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

pub type Wad = std::path::PathBuf;
pub type Param = String;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Iwad {
    Doom,
    Doom2,
}

#[derive(Serialize, Deserialize)]
pub enum Complevel {
    Doom19,
    Boom,
    MBF,
    Other(u8),
}

#[derive(Serialize, Deserialize)]
pub enum SourcePort {
    PrBoomPlus,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    name: String,
    iwad: Iwad,
    wads: Vec<Wad>,
    source_port: SourcePort,
    complevel: Option<Complevel>,
    extra_params: Option<Vec<Param>>,
}

impl Game {
    pub fn new(name: String) -> Self {
        Self {
            name,
            iwad: Iwad::Doom,
            wads: Vec::new(),
            source_port: SourcePort::PrBoomPlus,
            complevel: None,
            extra_params: None,
        }
    }

    pub fn with_iwad(&mut self, iwad: Iwad) -> &mut Game {
        self.iwad = iwad;
        self
    }

    pub fn with_wads(&mut self, wads: &[Wad]) -> &mut Game {
        self.wads = wads.to_vec();
        self
    }

    pub fn with_complevel(&mut self, complevel: Complevel) -> &mut Game {
        self.complevel = Some(complevel);
        self
    }

    pub fn with_params(&mut self, params: &[Param]) -> &mut Game {
        self.extra_params = Some(params.to_vec());
        self
    }

    pub fn save_game(&self) -> Result<()> {
        let project_dir = ProjectDirs::from("", "", "rdl")
            .ok_or_else(|| Error::msg("Failed to get project dir"))?;
        let game = ron::to_string(self)?;
        let mut file = File::create(
            project_dir
                .config_dir()
                .join("games")
                .join(format!("{}.ron", self.name)),
        )?;
        file.write_all(game.as_bytes()).map_err(Error::msg)
    }

    pub fn load_game(file: String) -> Result<Game> {
        ron::from_str(file.as_str()).map_err(|error| {
            let context = format!(
                "Failed to load game config.\nError code: {}\nPosition: {}",
                error.code, error.position
            );
            Error::new(error).context(context)
        })
    }
}
