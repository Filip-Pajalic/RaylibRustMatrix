use crate::animation::animation::Easing::EaseOut;
use raylib::prelude::Color;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum Easing {
    EaseIn,
    EaseOut,
    Linear,
}

#[derive(Debug)]
pub struct Transition {
    duration: Duration,
    easing: Easing,
}

#[derive(Debug)]
pub struct AnimationStep {
    color: Color,
    duration: Duration,
    transition: Option<Transition>,
}

#[derive(Debug)]
pub struct Animation {
    pub current_color: Color,
    timer: Instant,
    steps: Vec<AnimationStep>,
    current_step: usize,
}

impl Animation {
    pub fn new() -> Self {
        let mut anim = Animation {
            current_color: Color::WHITE,
            timer: Instant::now(),
            steps: vec![],
            current_step: 0,
        };
        anim.create_animation_steps();
        anim
    }

    fn create_animation_steps(&mut self) {
        let steps = vec![
            AnimationStep {
                color: Color::WHITE,
                duration: Duration::from_millis(100),
                transition: Some(Transition {
                    duration: Duration::from_millis(100),
                    easing: EaseOut,
                }),
            },
            AnimationStep {
                color: Color::GREEN,
                duration: Duration::from_millis(5000),
                transition: Some(Transition {
                    duration: Duration::from_millis(2000),
                    easing: EaseOut,
                }),
            },
            AnimationStep {
                color: Color::BLACK,
                duration: Duration::from_millis(500),
                transition: Some(Transition {
                    // Transition for the last step
                    duration: Duration::from_millis(2000),
                    easing: EaseOut,
                }),
            },
        ];
        self.steps = steps;
    }

    pub fn update(&mut self) {
        if self.steps.is_empty() {
            return;
        }
        self.update_color();
        self.advance_step_if_needed();
    }

    fn update_color(&mut self) {
        let current_step = &self.steps[self.current_step];

        if self.is_within_step_duration(current_step) {
            self.current_color = current_step.color;
        } else if !self.is_last_step() && current_step.transition.is_some() {
            self.current_color = self.calculate_transition_color(current_step);
        } else {
            self.current_color = current_step.color;
        }
    }

    fn calculate_transition_color(&self, current_step: &AnimationStep) -> Color {
        let elapsed_transition = self.timer.elapsed() - current_step.duration;
        let transition = current_step.transition.as_ref().unwrap();
        let t = elapsed_transition.as_secs_f32() / transition.duration.as_secs_f32();
        let eased_t = self.apply_easing(t, &transition.easing);
        let next_color = self.steps[self.current_step + 1].color;
        interpolate(current_step.color, next_color, eased_t)
    }

    fn apply_easing(&self, t: f32, easing: &Easing) -> f32 {
        match easing {
            Easing::EaseIn => ease_in(t),
            Easing::EaseOut => ease_out(t),
            Easing::Linear => t,
        }
    }

    fn advance_step_if_needed(&mut self) {
        if self.is_last_step() {
            return;
        }

        let current_step = &self.steps[self.current_step];
        let total_duration = self.calculate_total_duration(current_step);

        while self.timer.elapsed() >= total_duration && !self.is_last_step() {
            self.current_step += 1;
            self.timer = Instant::now();
        }
    }

    fn calculate_total_duration(&self, current_step: &AnimationStep) -> Duration {
        if !self.is_last_step() && current_step.transition.is_some() {
            current_step.duration + current_step.transition.as_ref().unwrap().duration
        } else {
            current_step.duration
        }
    }

    fn is_last_step(&self) -> bool {
        self.current_step == self.steps.len() - 1
    }

    fn is_within_step_duration(&self, current_step: &AnimationStep) -> bool {
        self.timer.elapsed() < current_step.duration
    }
}

fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

fn ease_in(t: f32) -> f32 {
    t * t
}

pub fn interpolate(start: Color, end: Color, factor: f32) -> Color {
    let r = (start.r as f32 * (1.0 - factor) + end.r as f32 * factor) as u8;
    let g = (start.g as f32 * (1.0 - factor) + end.g as f32 * factor) as u8;
    let b = (start.b as f32 * (1.0 - factor) + end.b as f32 * factor) as u8;
    Color::new(r, g, b, 255)
}
