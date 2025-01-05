use macroquad::{color, shapes::draw_line, window};

use crate::pendulum::Pendulum;

pub struct Database {
    pub kinetic_energy: [f32; 500],
    pub potential_energy: [f32; 500],
    pub mechanical_energy: [f32; 500],
    index: usize,
}

impl Database {
    pub fn new() -> Self {
        Self {
            kinetic_energy: [0.0; 500],
            potential_energy: [0.0; 500],
            mechanical_energy: [0.0; 500],
            index: 0,
        }
    }

    pub fn update(&mut self, pendulum: &Pendulum) {
        self.update_energies(pendulum);
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
        if self.index >= 500 {
            self.index = 0;
        }
    }

    pub fn draw(&self) {
        self.draw_energies();
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
}
