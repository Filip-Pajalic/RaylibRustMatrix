use crate::animation::Animation;
use crate::FONT_WIDTH;
use rand::distr::Distribution;
use rand::{Rng, RngCore};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::prelude::Font;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct MatrixCharacter {
    pub y_pos: i32,
    pub x_pos: i32,
    pub glyph: char,
    pub color: Color,
    pub max_depth: i32,
    pub child: Option<Box<MatrixCharacter>>,
    pub child_spawn_time: Instant,
    pub age: Instant,
    pub alive: bool,
    pub animation: Animation,
}

#[repr(u32)]
#[derive(Debug, Clone)]
enum MatrixColor {
    GREEN = 0x003B00F,
    WHITE = 0xFFFFFFF,
    BLACK = 0x0000000,
}

impl MatrixColor {
    fn to_color(self) -> Color {
        let hex = self as u32; // Extract the hex value
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color { r, g, b, a: 255 } // Default alpha to 255 (fully opaque)
    }
}

impl MatrixCharacter {
    pub fn new(y_pos: i32, x_pos: i32, max_depth: i32) -> Self {
        MatrixCharacter {
            y_pos,
            x_pos,
            glyph: '1',
            color: MatrixColor::WHITE.to_color(),
            child: None,
            max_depth,
            child_spawn_time: Instant::now(),
            age: Instant::now(),
            alive: true,
            animation: Animation::new(),
        }
    }

    fn spawn_child(&mut self) {
        if self.child.is_none()
            && self.y_pos <= self.max_depth
            && self.child_spawn_time.elapsed()
                > Duration::from_millis(crate::matrix::matrix::CHILD_SPAWN_TIME as u64)
        {
            self.child = Some(Box::new(MatrixCharacter::new(
                self.y_pos + 1,
                self.x_pos,
                self.max_depth.clone(),
            )))
        }
    }
    fn generate_random_character(&self) -> char {
        let mut rng = rand::rng();

        rng.sample(AlphanumericMatrix) as char
    }
    fn symbol_to_string(&self) -> String {
        self.glyph.to_string()
    }

    fn update(&mut self, d: &mut RaylibDrawHandle, font: &mut Font) {
        self.spawn_child();
        self.update_appearance();

        d.draw_text_ex(
            &font,
            &self.symbol_to_string(),
            Vector2::new(
                self.x_pos as f32 * FONT_WIDTH as f32,
                (self.y_pos * font.baseSize) as f32,
            ),
            font.baseSize as f32,
            2.0,
            self.color,
        );
    }

    fn update_appearance(&mut self) {
        self.glyph = self.generate_random_character();

        // let t = current_age.as_secs_f32() / self.max_depth.clone() as f32;
        self.animation.update();
        self.color = self.animation.current_color;
    }

    pub fn traverse_and_tick(&mut self, d: &mut RaylibDrawHandle, font: &mut Font) {
        if let Some(ref mut child) = self.child {
            child.traverse_and_tick(d, font);
        }
        self.update(d, font);
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AlphanumericMatrix;

impl Distribution<u8> for AlphanumericMatrix {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 10 + 9;

        const GEN_ASCII_STR_CHARSET: &[u8] = b"EIOPQRTUWY\
                012345789";
        // We can pick from 62 characters. This is so close to a power of 2, 64,
        // that we can do better than `Uniform`. Use a simple bitshift and
        // rejection sampling. We do not use a bitmask, because for small RNGs
        // the most significant bits are usually of higher quality.
        loop {
            let var = rng.next_u32() >> (32 - 6);
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}
