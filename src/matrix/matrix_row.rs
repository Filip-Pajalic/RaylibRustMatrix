use std::time::{Duration, Instant};
use crate::matrix::matrix_character::MatrixCharacter;
use crate::FONT_HEIGHT;
use rand::Rng;
use raylib::drawing::RaylibDrawHandle;
use raylib::ffi::rlDrawCall;
use raylib::prelude::Font;


pub struct Timer{
    elapsed: Instant,
    end: Duration,
}

#[derive(Debug)]
pub struct MatrixRow {
    pub x_pos: i32,
    pub code: MatrixCharacter,
    pub is_spawned: bool,
    pub y_pos: i32,
    pub time_elapsed: Instant,
    pub start_delay: Duration,
    pub window_width: i32,
    pub  is_timererd: bool
}

impl MatrixRow {
    pub fn new(x_pos: i32, ytotal: i32, window_width: i32) -> Self {
        let y_pos = Self::calculate_y_values(window_width);
        let matrix_character = MatrixCharacter::new(y_pos, x_pos, Self::random_length(ytotal));
        let start_duration = Self::random_duration(Duration::from_millis(100), Duration::from_millis(100000));
        MatrixRow {
            x_pos,
            code: matrix_character,
            is_spawned: true,
            is_timererd: false,
            y_pos: 0,
            time_elapsed : Instant::now(),
            start_delay: start_duration,
            window_width,
        }
    }
    pub fn update(&mut self, d: &mut RaylibDrawHandle, font: &mut Font) {
        if self.time_elapsed.elapsed() > self.start_delay{
            self.is_timererd = true;
        }

       if self.is_timererd {
            self.code.traverse_and_tick(d, font);
            if self.code.alive == false {
                self.is_spawned = false;
            }
        }

    }
    fn random_duration(min: Duration, max: Duration) -> Duration {
        let mut rng = rand::rng();
        let range = max.as_millis() - min.as_millis();
        let random_millis = rng.random_range(0..=range) as u64;
        min + Duration::from_millis(random_millis)
    }

    fn random_length(max: i32) -> i32{

        let mut rng = rand::rng();
        rng.random_range(10..=max/2)
    }
    fn calculate_y_values(width: i32) -> i32 {
        let mut rng = rand::rng();
        let min = -3;
        let max = width / FONT_HEIGHT - 3;
        rng.random_range(min..=max)
    }
}
