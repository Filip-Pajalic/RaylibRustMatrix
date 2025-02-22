use raylib::prelude::*;

mod matrix;

use matrix::*;


const FONT_HEIGHT: i32 = 20;
const FONT_WIDTH: i32 = 12;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;



fn main() {
    let mut matrix = MatrixWorld::new();
    let (mut rl, thread) = raylib::init().size(WINDOW_WIDTH, WINDOW_HEIGHT).title("Hello, World").build();
    let mut count = 0;
    rl.set_target_fps(2);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        count += 1;
        d.clear_background(Color::WHITE);

        matrix.tick(&mut d);
    }
}


