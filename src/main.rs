use rand::Rng;
use raylib::prelude::*;
fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    let mut count = 0;
    rl.set_target_fps(2);
    let matrix = Matrix::new(10, 2);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        count += 1;
        d.clear_background(Color::WHITE);

        for row in matrix.rows.iter().clone() {
            for item in row.text.iter().clone() {
                let text = item.value.to_string();
                let converted = &text[..];
                d.draw_text(
                    converted,
                    row.x as i32 * 20,
                    item.y as i32 * 12 + count * 12,
                    20,
                    Color::BLACK,
                );
            }
        }
    }
}

struct MatrixText {
    y: u32,
    value: char,
}

struct MatrixRow {
    x: u32,
    text: Vec<MatrixText>,
}

struct Matrix {
    width: u32,
    height: u32,
    rows: Vec<MatrixRow>,
}

impl Matrix {
    fn new(width: u32, height: u32) -> Self {
        let mut matrix_rows: Vec<MatrixRow> = vec![];
        for x_pos in 0..width {
            matrix_rows.push(MatrixRow::new(x_pos, height));
        }
        Matrix {
            width: width,
            height: height,
            rows: matrix_rows,
        }
    }

    fn tick(&self) {
        todo!("Update stuff")
    }
}

impl MatrixRow {
    fn new(x_pos: u32, ytotal: u32) -> Self {
        let mut text_total: Vec<MatrixText> = vec![];
        for y_pos in 0..ytotal {
            text_total.push(MatrixText::new(y_pos));
        }
        MatrixRow {
            x: x_pos,
            text: text_total,
        }
    }
    fn tick(&self) {
        todo!("Update stuff")
    }
}

impl MatrixText {
    fn new(y_pos: u32) -> Self {
        let mut text = MatrixText {
            y: y_pos,
            value: '1',
        };
        text.value = text.generate_random_character();
        text
    }
    fn generate_random_character(&self) -> char {
        // Get an RNG:
        let mut rng = rand::rng();
        // Try printing a random alphanumeric value instead!
        rng.sample(rand::distr::Alphanumeric) as char
    }
    fn tick(&self) {
        todo!("Update stuff")
    }
}
