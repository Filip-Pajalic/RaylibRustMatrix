use std::ptr::null;
use rand::distr::Uniform;
use rand::Rng;
use raylib::prelude::*;
use crate::FONT_HEIGHT;
use crate::FONT_WIDTH;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;


#[derive(Debug)]
pub struct MatrixWorld {
    pub width: i32,
    pub height: i32,
    pub rows: Vec<MatrixCode>,
}

impl MatrixWorld {
    pub fn new() -> Self {

        let mut matrix = MatrixWorld {
            width: 0,
            height: 0,
            rows: vec![],
        };
        matrix.calculate_font_grid_size();
        for i in 0..matrix.width{
            matrix.rows.push(MatrixCode::new(i,matrix.height));
        }
        matrix
    }

    pub fn tick(&mut self,d: &mut RaylibDrawHandle) {
        for i in 0..self.width{
            self.spawn_row(i);
        }
        for mut row in self.rows.iter_mut() {
            row.tick(d);
        }
    }

    fn calculate_font_grid_size(&mut self){
        let width:i32 = WINDOW_WIDTH / FONT_WIDTH;
        let height:i32 = WINDOW_HEIGHT / FONT_HEIGHT;
        self.height = height as i32;
        self.width = width as i32;
    }

    fn spawn_row(&mut self, index:i32) {
        if let Some(row) = self.rows.get_mut(index as usize){
            if !row.is_spawned {
                *row = MatrixCode::new(index, self.height);
            }
        }
    }

}

#[derive(Debug)]
pub struct MatrixCode {
    pub x_pos: i32,
    pub code:MatrixCharacter,
    pub is_spawned: bool,
    pub y_pos: i32,
    pub start_delay: i32,
}

impl MatrixCode {
    pub fn new(x_pos: i32, ytotal: i32) -> Self {
        let y_pos = Self::calculate_y_values();
        let matrix_character = MatrixCharacter::new(y_pos,x_pos, ytotal);

        MatrixCode {
            x_pos,
            code: matrix_character,
            is_spawned: true,
            y_pos: 0,
            start_delay: 0,
        }
    }
    pub fn tick(&mut self , d: &mut RaylibDrawHandle) {
        if self.is_spawned {
            self.code.traverse_and_tick(d);
        }

    }
    fn calculate_y_values()-> i32 {
        let mut rng = rand::rng();
        let min = -10;
        let max = WINDOW_HEIGHT-200;
        rng.random_range(min..=max)
    }
}

#[derive(Debug)]
pub struct MatrixCharacter {
    pub y_pos: i32,
    pub x_pos: i32,
    pub symbol: char,
    pub color : Color,
    pub max_depth: i32,
    pub child: Option<Box<MatrixCharacter>>,
}


impl MatrixCharacter {
    pub fn new(y_pos: i32, x_pos: i32, max_depth: i32) -> Self {
        let mut text = MatrixCharacter {
            y_pos,
            x_pos,
            symbol: '1',
            color: Color::GREEN,
            child: None,
            max_depth
        };
        text
    }

    fn spawn_child(&mut self){
        if self.y_pos <= self.max_depth {
            self.child = Some(Box::new(MatrixCharacter::new(self.y_pos + 1, self.x_pos,self.max_depth.clone())))
        }
    }
    fn generate_random_character(&self) -> char {
        // Get an RNG:
        let mut rng = rand::rng();
        // Try printing a random alphanumeric value instead!
        rng.sample(rand::distr::Alphanumeric) as char
    }
    fn get_str(&self) -> String { // Change return type to `String`
        self.symbol.to_string() // Convert `char` to `String`
    }

    fn tick(&mut self,d: &mut RaylibDrawHandle) {
        self.spawn_child();
        self.symbol = self.generate_random_character();

        let offset = 10;

        d.draw_text(

            &self.get_str(),
            self.x_pos as i32 * FONT_HEIGHT,
            (self.y_pos as i32+ offset ) * FONT_WIDTH,
            19,
            self.color,
        );
    }

    pub fn traverse_and_tick(&mut self, d: &mut RaylibDrawHandle){
        self.tick(d);
        if let Some(ref mut child) = self.child{
            child.traverse_and_tick(d);
        }
    }
}
