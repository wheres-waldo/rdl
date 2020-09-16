use serde::{Deserialize, Serialize};

pub type Wad = std::path::PathBuf;
pub type Param = String;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Iwad {
    Doom,
    Doom2,
}

pub enum Complevel {
    Doom19,
    Boom,
    MBF,
    Other(u8),
}

pub enum SourcePort {
    PrBoomPlus,
}

pub struct Game {
    name: String,
    iwad: Iwad,
    wads: Vec<Wad>,
    source_port: SourcePort,
    complevel: Option<Complevel>,
    extra_params: Option<Vec<Param>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            iwad: Iwad::Doom,
            wads: Vec::new(),
            source_port: SourcePort::PrBoomPlus,
            complevel: None,
            extra_params: None,
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
