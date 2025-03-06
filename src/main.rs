mod animation;
mod config;
mod game;
mod matrix;

use raylib::prelude::*;

use crate::config::Config;
use crate::game::GameState;
use matrix::*;

const FONT_HEIGHT: i32 = 30;
const FONT_WIDTH: i32 = 30;

fn main() {
    let config = Config::from_file("config.yaml");
    let game_state = GameState::new(config);
    let mut matrix = MatrixWorld::new(game_state.config.clone());

    let config_read = game_state.config.read().unwrap();
    let window_width = config_read.world.window_width;
    let window_height = config_read.world.window_height;

    let (mut rl, thread) = init()
        .size(window_width, window_height)
        .title("Matrix")
        .build();
    rl.set_target_fps(10);
    let mut font = rl.load_font(&thread, "resources/matrix-code.ttf").unwrap();
    font.baseSize = config_read.world.font_size.clone();
    rl.gui_enable();

    drop(config_read);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        matrix.update(&mut d, &mut font);
    }
}
