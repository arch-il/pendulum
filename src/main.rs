use core::f32;

use macroquad::{color, shapes::draw_line, window};
use pendulum::Pendulum;

mod pendulum;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Pendulum Simulation".to_owned(),
        window_resizable: false,
        window_width: 1000, //? I need 2x size when high_dpi is true?
        window_height: 1000,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let pendulum = Pendulum::new(
        vec![-f32::consts::PI / 2.0, f32::consts::PI, f32::consts::PI],
        vec![50., 50., 50.],
    );

    loop {
        window::clear_background(color::BLACK);

        let mut points = pendulum.points.iter().peekable();
        while let Some(current) = points.next() {
            if let Some(next) = points.peek() {
                draw_line(current.x, current.y, next.x, next.y, 5.0, color::LIME);
            }
        }

        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        window::next_frame().await
    }
}
