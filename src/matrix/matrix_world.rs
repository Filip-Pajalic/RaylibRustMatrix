use crate::FONT_WIDTH;
use std::sync::{Arc, RwLock};

use crate::config::Config;
use crate::matrix::matrix_row::MatrixRow;
use raylib::prelude::*;

pub(crate) const CHILD_SPAWN_TIME: usize = 500;

const MIN_FONT_SIZE: i32 = 10;
const MAX_FONT_SIZE: i32 = 200;

#[derive(Debug)]
pub struct MatrixWorld {
    pub width: i32,
    pub height: i32,
    pub font_height: i32,
    pub font_width: i32,
    pub height_internal: i32,
    pub rows: Vec<MatrixRow>,
    pub config: Arc<RwLock<Config>>,
}

impl MatrixWorld {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        let mut matrix = MatrixWorld {
            width: 0,
            height: 0,
            rows: vec![],
            config,
            font_height: 100,
            height_internal: 0, //height+200,
            font_width: 10,     //fix some proper value here
        };
        matrix.font_height = matrix.config.read().unwrap().world.font_size_px;
        matrix.calculate_font_grid_size();
        for i in 0..matrix.width {
            matrix
                .rows
                .push(MatrixRow::new(i, matrix.height, matrix.width));
        }

        matrix
    }

    pub fn update(&mut self, d: &mut RaylibDrawHandle, font: &mut Font) {
        for i in 0..self.width {
            self.spawn_row(i);
        }
        for row in self.rows.iter_mut() {
            row.update(d, font);
        }
        if self.config.read().unwrap().debug {
            self.debug_grid(d, font);
        }
    }

    fn calculate_grid_size(&self) -> (i32, i32) {
        let grid_size_x = self.width / self.font_width;
        let grid_size_y = self.height / self.font_height;
        (grid_size_x, grid_size_y)
    }
    fn get_grid_offset(&self) -> (i32, i32) {
        let grid_size_x_remainder = (self.width % self.font_width) / 2;
        let grid_size_y_remainder = (self.height % self.font_height) / 2;

        (grid_size_x_remainder, grid_size_y_remainder)
    }

    fn update_font_width(&mut self, font: &Font) {
        let widest_char = self.get_widest_char_width(font);
        let font_width_ratio = widest_char / self.font_height as f32;
        self.font_width = (self.font_height as f32 * font_width_ratio).round() as i32;
    }

    fn update_font_size(&mut self, increase: bool, font: &Font) {
        if increase {
            if self.font_height < MAX_FONT_SIZE {
                self.font_height += 5;
            }
        } else {
            if self.font_height > MIN_FONT_SIZE {
                self.font_height -= 5;
            }
        }
        self.update_font_width(font);
    }
    fn get_widest_char_width(&self, font: &Font) -> f32 {
        let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                1234567890";
        characters
            .chars()
            .map(|c| self.get_character_width(&font, &c.to_string()))
            .fold(0.0, f32::max)
    }

    fn get_character_width(&self, font: &Font, character: &str) -> f32 {
        font.measure_text(character, self.font_height as f32, 0.0).x
    }
    fn get_character_offset(&self, font: &Font, character: &str) -> f32 {
        let width = self.get_character_width(font, character);
        (self.get_widest_char_width(font) - width) / 2_f32
    }
    fn get_height_internal_offset_range(&self) -> (i32, i32) {
        let range = (self.height_internal - self.height).abs() / 2;
        (-range, range)
    }

    pub fn debug_grid(&mut self, d: &mut RaylibDrawHandle, font: &mut Font) {
        let (mut grid_size_x, mut grid_size_y) = self.calculate_grid_size();
        let (mut grid_offset_x, mut grid_offset_y) = self.get_grid_offset();
        self.update_font_width(font);

        let (min, max) = self.get_height_internal_offset_range();
        //let offset = self.get_character_offset(font, "A"); // so the grid is not over /font offset
        for x in min..grid_size_x + max {
            for y in min..grid_size_y + max {
                d.draw_rectangle_lines(
                    grid_offset_x + x * self.font_width,
                    grid_offset_y + y * self.font_height,
                    self.font_width,
                    self.font_height,
                    Color::RED,
                );
            }
        }
    }

    fn calculate_font_grid_size(&mut self) {
        if let Ok(cfg) = self.config.read() {
            self.width = cfg.world.window_width_px / 10;
            self.height = cfg.world.window_height_px / 10;
        }
    }

    fn spawn_row(&mut self, index: i32) {
        if let Some(row) = self.rows.get_mut(index as usize) {
            if !row.is_spawned {
                *row = MatrixRow::new(index, self.height, self.width);
            }
        }
    }
}
