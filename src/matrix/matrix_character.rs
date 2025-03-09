use crate::animation::Animation;
use crate::FONT_WIDTH;
use rand::distr::Distribution;
use rand::{ Rng, RngCore};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::prelude::Font;
use std::time::{Duration, Instant};
use crate::animation::animation::AnimationStep;

#[derive(Debug)]
pub struct MatrixCharacter {
    pub y_pos: i32,
    pub x_pos: i32,
    pub glyph: char,
    pub color: Color,
    pub max_child_depth: i32,
    pub child: Option<Box<MatrixCharacter>>,
    pub child_spawn_time: Instant,
    pub age: Instant,
    pub glyph_change_timer: Instant,
    pub glyph_change_interval: Duration,
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
    fn to_color(self, alpha: u8) -> Color {
        let hex = self as u32; // Extract the hex value
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color { r, g, b, a: alpha } // Default alpha to 255 (fully opaque)
    }
}

impl MatrixCharacter {
    pub fn new(y_pos: i32, x_pos: i32, max_depth: i32) -> Self {
        let duration = random_duration(Duration::from_millis(100), Duration::from_millis(700));

        let mut rng = rand::rng();
        let transparency = rng.random_range(50..=255) as u8;
        let mut matrix_char = MatrixCharacter {
            y_pos,
            x_pos,
            glyph: '1',
            color: MatrixColor::WHITE.to_color(255),
            child: None,
            max_child_depth: max_depth,
            child_spawn_time: Instant::now(),
            age: Instant::now(),
            glyph_change_timer: Instant::now(),
            glyph_change_interval: duration,
            alive: true,
            animation: Animation::new(Self::create_animation_steps(transparency)),
        };
        matrix_char.glyph = matrix_char.generate_random_character();
        matrix_char
    }

    fn spawn_child(&mut self) {
        if self.child.is_none()
            && self.y_pos <= self.max_child_depth
            && self.child_spawn_time.elapsed()
                > Duration::from_millis(crate::matrix::matrix_world::CHILD_SPAWN_TIME as u64)
        {
            self.child = Some(Box::new(MatrixCharacter::new(
                self.y_pos + 1,
                self.x_pos,
                self.max_child_depth.clone(),
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
        if self.glyph_change_timer.elapsed() >= self.glyph_change_interval {
            self.glyph = self.generate_random_character();
            self.glyph_change_timer = Instant::now()
        }

        // let t = current_age.as_secs_f32() / self.max_depth.clone() as f32;
        self.animation.update();
        self.color = self.animation.current_color;
    }

    pub fn traverse_and_tick(&mut self, d: &mut RaylibDrawHandle, font: &mut Font) {
        if let Some(ref mut child) = self.child {
            child.traverse_and_tick(d, font);
            if !child.alive {
                if self.animation.concluded {
                    self.alive = false;
                }
            }
        } else {
            if self.animation.concluded {
                self.alive = false;
            }
        }
        self.update(d, font);
    }
    fn create_animation_steps(alpha: u8) -> Vec<AnimationStep> {
        let steps = vec![
            AnimationStep {
                color: Color::WHITE,
                duration: Duration::from_millis(100),
                transition: Some(crate::animation::animation::Transition {
                    duration: None,
                    easing: None,
                    triggered: true,
                }),
            },
            AnimationStep {
                color: MatrixColor::GREEN.to_color(alpha),
                duration: Duration::from_millis(3000),
                transition: Some(crate::animation::animation::Transition {
                    duration: Some(Duration::from_millis(1000)),
                    easing: Some(crate::animation::animation::Easing::EaseOut),
                    triggered: false,
                }),
            },
            AnimationStep {
                color: Color::BLACK,
                duration: Duration::from_millis(500),
                transition: Some(crate::animation::animation::Transition {
                    // Transition for the last step
                    duration: Some(Duration::from_millis(2000)),
                    easing: Some(crate::animation::animation::Easing::EaseOut),
                    triggered: false,
                }),
            },
        ];
       steps
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

fn random_duration(min: Duration, max: Duration) -> Duration {
    let mut rng = rand::rng();
    let range = max.as_millis() - min.as_millis();
    let random_millis = rng.random_range(0..=range) as u64;
    min + Duration::from_millis(random_millis)
}
