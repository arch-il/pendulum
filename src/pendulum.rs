use macroquad::math::Vec2;

pub struct Pendulum {
	pub p1: Vec2,
	pub p2: Vec2,
	pub p3: Vec2,
}

impl Pendulum {
	pub fn new() -> Self {
		Self {
			p1: Vec2::new(0.0, 0.0),
			p2: Vec2::new(100.0, 100.0),
			p3: Vec2::new(200.0, 100.0),
		}
	}
}