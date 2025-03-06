use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub world: World,
    pub row: Row,
    pub glyph: Glyph,
    pub debug: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct World {
    pub window_height: i32,
    pub window_width: i32,
    pub font_size: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Row {
    pub base_spawn_time: i32,
    pub padding: i32,
    pub bounding_box_offset_y: Vec<i32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Glyph {
    pub child_spawn_time: i32,
    pub max_age: i32,
    pub padding: i32,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read config file");
        serde_yaml::from_str(&content).expect("Failed to parse config file")
    }
}
