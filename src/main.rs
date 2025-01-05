use core::f32;

use macroquad::{color, shapes::draw_line, time, window};
use pendulum::Pendulum;

mod pendulum;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Pendulum Simulation".to_owned(),
        window_resizable: false,
        window_width: 1000,  // twice size due to 2x dpi
        window_height: 1000, // twice size due to 2x dpi
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut pendulum = Pendulum::new(
        vec![-f32::consts::PI / 2.0, f32::consts::PI, f32::consts::PI],
        vec![50., 50., 50.],
    );

    loop {
        window::clear_background(color::BLACK);

        let mut points = pendulum.cords.iter().peekable();
        while let Some(current) = points.next() {
            if let Some(next) = points.peek() {
                draw_line(current.x, current.y, next.x, next.y, 5.0, color::LIME);
            }
        }

        pendulum.tick(time::get_frame_time());
        pendulum.tick(time::get_frame_time());
        pendulum.tick(time::get_frame_time());
        pendulum.tick(time::get_frame_time());
        pendulum.tick(time::get_frame_time());

        window::next_frame().await
    }
}
