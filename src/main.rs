use database::Database;
use macroquad::{color, shapes::draw_line, time, window};
use pendulum::Pendulum;

mod database;
mod pendulum;
mod vec2;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Pendulum Simulation".to_owned(),
        window_resizable: false,
        window_width: 500,
        window_height: 500,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut pendulum = Pendulum::new(vec![0.0, 0.0, 0.0], vec![75., 75., 75.]);
    let mut database = Database::new();

    loop {
        window::clear_background(color::BLACK);

        // draw pendulum
        let mut points = pendulum.cords.iter().peekable();
        while let Some(current) = points.next() {
            if let Some(next) = points.peek() {
                draw_line(
                    current.x as f32,
                    current.y as f32,
                    next.x as f32,
                    next.y as f32,
                    5.0,
                    color::LIME,
                );
            }
        }

        database.draw();

        // updates
        let dt = time::get_frame_time();
        pendulum.update(dt);

        database.update(&pendulum, dt);

        window::next_frame().await
    }
}
