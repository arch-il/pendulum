use macroquad::{math::Vec2, window};

pub struct Pendulum {
    pub points: Vec<Vec2>,
}

impl Pendulum {
    pub fn new(angles: Vec<f32>, lengths: Vec<f32>) -> Self {
        let mut p = Vec2::new(window::screen_width() / 2.0, window::screen_height() / 2.0);
        let mut points = Vec::new();
        points.push(p);
        for (&a, &l) in angles.iter().zip(lengths.iter()) {
            p += Vec2::new(l * f32::cos(a), l * f32::sin(a));
            points.push(p);
        }
        Self { points }
    }
}
