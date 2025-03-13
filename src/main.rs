mod animation;
mod config;
mod game;
mod matrix;

use raylib::prelude::*;
use std::process;

use crate::config::Config;
use crate::game::GameState;
use matrix::*;

const FONT_HEIGHT: i32 = 30;
const FONT_WIDTH: i32 = 30;

fn main() -> Result<(), MatrixError> {
    let config = Config::from_file("config.yaml");
    let game_state = GameState::new(config);

    let matrix = MatrixWorld::new(game_state.config.clone());
    match matrix {
        Ok(mut matrix) => {
            let config_read = game_state.config.read().unwrap();
            let window_width = config_read.world.window_width_px;
            let window_height = config_read.world.window_height_px;

            let (mut rl, thread) = init()
                .size(window_width, window_height)
                .title("Matrix")
                .build();
            rl.set_target_fps(10);
            let mut font = rl.load_font(&thread, "resources/matrix-code.ttf").unwrap();
            font.baseSize = config_read.world.font_size_px.clone();
            rl.gui_enable();

            drop(config_read);
            while !rl.window_should_close() {
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                matrix.update(&mut d, &mut font);
            }
            Ok(())
        }
        Err(error) => {
            eprintln!("Error creating matrix: {}", error);
            process::exit(1)
        }
    }
}
