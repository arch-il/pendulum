use macroquad::window;

use crate::vec2::Vec2;

pub struct Pendulum {
    pub cords: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    pub lengths: Vec<f32>,
}

impl Pendulum {
    pub fn new(angles: Vec<f32>, lengths: Vec<f32>) -> Self {
        let mut p = Vec2::new(
            (window::screen_width() / 2.0) as f64,
            (window::screen_height() / 3.0) as f64,
        );
        let mut cords = Vec::new();
        cords.push(p);
        for (&a, &l) in angles.iter().zip(lengths.iter()) {
            p += Vec2::new((l * f32::cos(a)) as f64, (l * f32::sin(a)) as f64);
            cords.push(p);
        }
        Self {
            cords,
            lengths,
            velocities: vec![Vec2::new(0.0, 0.0); angles.len()],
        }
    }

    pub fn update(&mut self, dt: f64) {
        const TARGET_STEP: f64 = 0.000001;
        for _ in 0..(dt / TARGET_STEP) as usize {
            self.tick(TARGET_STEP);
        }
    }

    fn tick(&mut self, dt: f64) {
        const G: f64 = 980.0;
        let pen_num = self.velocities.len();
        let mut p_cords = Vec::new();

        for i in 0..pen_num {
            self.velocities[i].y += G * dt;

            p_cords.push(self.cords[i + 1]);

            self.cords[i + 1] += self.velocities[i] * dt;
        }

        for i in 0..pen_num {
            let dv = self.cords[i + 1] - self.cords[i];
            let d = dv.length();

            let corr = (self.lengths[i] as f64 - d) / d;
            if i == 0 {
                self.cords[i + 1] += dv * corr;
            } else {
                self.cords[i] -= dv * (corr / 2.0);
                self.cords[i + 1] += dv * (corr / 2.0);
            }
        }

        for (i, v) in self.velocities.iter_mut().enumerate() {
            *v = (self.cords[i + 1] - p_cords[i]) / dt;
        }
    }
}
