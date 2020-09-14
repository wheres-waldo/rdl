use Complevel::*;
use Iwad::*;

type Wad = std::path::PathBuf;
type Param = String;

pub enum Iwad {
    Doom,
    Doom2,
}

pub enum Complevel {
    Doom,
    Boom,
    MBF,
    Other(u8),
}

pub struct Game {
    name: String,
    iwad: Iwad,
    wads: Vec<Wad>,
    complevel: Complevel,
    extra_params: Vec<Param>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            iwad: Doom,
            wads: Vec::new(),
            complevel: Doom,
            extra_params: Vec::new(),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
