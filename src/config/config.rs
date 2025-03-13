use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub world: World,
    pub glyph: Glyph,
    pub debug: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct World {
    pub window_width_px: i32,
    pub window_height_px: i32,
    pub font_size_px: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Glyph {
    pub child_spawn_interval_ms: i32,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read config file");
        serde_yaml::from_str(&content).expect("Failed to parse config file")
    }
}
