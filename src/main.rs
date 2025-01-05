use macroquad::prelude::*;
use pendulum::Pendulum;

mod pendulum;

#[macroquad::main("BasicShapes")]
async fn main() {
    let pendulum = Pendulum::new();

    loop {
        clear_background(BLACK);
        draw_line(pendulum.p1.x, pendulum.p1.y, pendulum.p2.x, pendulum.p2.y, 5.0, LIME);
        draw_line(pendulum.p2.x, pendulum.p2.y, pendulum.p3.x, pendulum.p3.y, 5.0, LIME);
        
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}