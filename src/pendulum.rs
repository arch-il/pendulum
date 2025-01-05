use macroquad::{math::Vec2, window};

pub struct Pendulum {
    pub cords: Vec<Vec2>,
    pub velocities: Vec<Vec2>,
    pub lengths: Vec<f32>,
}

impl Pendulum {
    pub fn new(angles: Vec<f32>, lengths: Vec<f32>) -> Self {
        let mut p = Vec2::new(window::screen_width() / 2.0, window::screen_height() / 3.0);
        let mut cords = Vec::new();
        cords.push(p);
        for (&a, &l) in angles.iter().zip(lengths.iter()) {
            p += Vec2::new(l * f32::cos(a), l * f32::sin(a));
            cords.push(p);
        }
        Self {
            cords,
            lengths,
            velocities: vec![Vec2::ZERO; angles.len()],
        }
    }

    pub fn update(&mut self, dt: f32) {
        const TARGET_STEP: f32 = 0.001;
        for _ in 0..(dt / TARGET_STEP) as usize {
            self.tick(TARGET_STEP);
        }
    }

    fn tick(&mut self, dt: f32) {
        const G: f32 = 980.0;
        let pen_num = self.velocities.len();
        let mut p_cords = Vec::new();

        for i in 0..pen_num {
            self.velocities[i].y += G * dt;

            p_cords.push(self.cords[i + 1].clone());

            self.cords[i + 1] += self.velocities[i] * dt;
        }

        for i in 0..pen_num {
            let dv = self.cords[i + 1] - self.cords[i];
            let d = dv.length();

            let corr = (self.lengths[i] - d) / d;
            if i == 0 {
                self.cords[i + 1] += corr * dv;
            } else {
                self.cords[i] -= (corr / 2.0) * dv;
                self.cords[i + 1] += (corr / 2.0) * dv;
            }
        }

        for i in 0..pen_num {
            self.velocities[i] = (self.cords[i + 1] - p_cords[i]) / dt;
        }
    }
}
