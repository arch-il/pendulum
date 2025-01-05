use core::f32;
use std::f32::consts::PI;

use macroquad::{
    color,
    shapes::draw_line,
    text::draw_text,
    window::{self, screen_height, screen_width},
};

use crate::pendulum::Pendulum;

pub struct Database {
    pub kinetic_energy: [f32; 500],
    pub potential_energy: [f32; 500],
    pub mechanical_energy: [f32; 500],
    index: usize,

    pub theta: Vec<f32>, // angle
    pub alpha: Vec<f32>, // angular acceleration
    p_theta: f32,
}

impl Database {
    pub fn new() -> Self {
        Self {
            kinetic_energy: [0.0; 500],
            potential_energy: [0.0; 500],
            mechanical_energy: [0.0; 500],
            index: 0,

            theta: Vec::new(),
            alpha: Vec::new(),
            p_theta: 0.0,
        }
    }

    pub fn update(&mut self, pendulum: &Pendulum, dt: f32) {
        self.update_energies(pendulum);
        self.update_phase_space(pendulum, dt);
    }

    fn update_phase_space(&mut self, pendulum: &Pendulum, dt: f32) {
        let vec =
            *pendulum.cords.last().unwrap() - *pendulum.cords.iter().rev().skip(1).next().unwrap();
        let mut theta = f32::atan2(vec.y as f32, vec.x as f32);

        if theta - self.p_theta > PI {
            theta -= 2.0 * PI;
        } else if theta - self.p_theta < -PI {
            theta += 2.0 * PI;
        }

        let alpha = (theta - self.p_theta) / dt;
        self.p_theta = theta;

        if alpha.abs() > 100.0 || alpha == 0.0 {
            return;
        }

        self.theta.push(theta);
        self.alpha.push(alpha);
    }

    fn update_energies(&mut self, pendulum: &Pendulum) {
        self.kinetic_energy[self.index] = pendulum
            .velocities
            .iter()
            .fold(0.0, |acc, v| acc + (v.length() as f32 / 100.0).powi(2))
            / 2.0;

        self.potential_energy[self.index] = pendulum.cords.iter().fold(0.0, |acc, c| {
            acc + (window::screen_height() - c.y as f32) / 100.0 * 9.8
        });

        self.mechanical_energy[self.index] =
            self.kinetic_energy[self.index] + self.potential_energy[self.index];

        self.index += 1;
        if self.index >= screen_width() as usize {
            self.index = 0;
        }
    }

    pub fn draw(&self) {
        draw_text(
            &format!(
                "{}; {}; {};",
                self.mechanical_energy[if self.index == 0 { 499 } else { self.index - 1 }],
                self.theta.last().unwrap_or(&0.0),
                self.alpha.last().unwrap_or(&0.0),
            ),
            5.0,
            12.0,
            20.0,
            color::LIGHTGRAY,
        );
        self.draw_energies();
        self.draw_phase_space();
    }

    fn draw_energies(&self) {
        let mut energies = self
            .kinetic_energy
            .iter()
            .zip(self.potential_energy.iter())
            .zip(self.mechanical_energy.iter())
            .rev()
            .enumerate()
            .peekable();
        while let Some((curr_i, ((&curr_k, &curr_p), &curr_m))) = energies.next() {
            if let Some((next_i, ((next_k, next_p), next_m))) = energies.peek() {
                if *next_i == 500 - self.index || curr_i == 500 - self.index {
                    continue;
                }

                const SHRINK: f32 = 0.5;
                draw_line(
                    window::screen_width() - curr_i as f32,
                    window::screen_height() - curr_k * SHRINK,
                    window::screen_width() - *next_i as f32,
                    window::screen_height() - **next_k * SHRINK,
                    1.0,
                    color::RED,
                );
                draw_line(
                    window::screen_width() - curr_i as f32,
                    window::screen_height() - curr_p * SHRINK,
                    window::screen_width() - *next_i as f32,
                    window::screen_height() - **next_p * SHRINK,
                    1.0,
                    color::BLUE,
                );
                draw_line(
                    window::screen_width() - curr_i as f32,
                    window::screen_height() - curr_m * SHRINK,
                    window::screen_width() - *next_i as f32,
                    window::screen_height() - **next_m * SHRINK,
                    1.0,
                    color::PURPLE,
                );
            }
        }
    }

    fn draw_phase_space(&self) {
        let mut iter = self
            .theta
            .iter()
            .zip(self.alpha.iter())
            .rev()
            .take(1000)
            .peekable();

        const SCALE_V: f32 = 25.0;
        const SCALE_H: f32 = 50.0;

        // ength of pi
        draw_line(
            screen_width() / 2.0,
            screen_height() / 2.0,
            screen_height() / 2.0 + PI * SCALE_H,
            screen_height() / 2.0,
            5.0,
            color::RED,
        );

        while let Some((&t, &a)) = iter.next() {
            if let Some((&nt, &na)) = iter.peek() {
                draw_line(
                    t * SCALE_H + screen_width() / 2.0,
                    a * SCALE_V + screen_height() / 2.0,
                    nt * SCALE_H + screen_width() / 2.0,
                    na * SCALE_V + screen_height() / 2.0,
                    1.0,
                    color::GRAY,
                );
            }
        }
    }
}
