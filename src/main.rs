use macroquad::{color, shapes::draw_line, window};
use pendulum::Pendulum;

mod pendulum;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Pendulum Simulation".to_owned(),
        window_resizable: false,
        window_width: 500,
        window_height: 500,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let pendulum = Pendulum::new();

    loop {
        window::clear_background(color::BLACK);
        draw_line(
            pendulum.p1.x,
            pendulum.p1.y,
            pendulum.p2.x,
            pendulum.p2.y,
            5.0,
            color::LIME,
        );
        draw_line(
            pendulum.p2.x,
            pendulum.p2.y,
            pendulum.p3.x,
            pendulum.p3.y,
            5.0,
            color::LIME,
        );

        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        window::next_frame().await
    }
}
