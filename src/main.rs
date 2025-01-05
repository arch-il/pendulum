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
    let mut pendulum = Pendulum::new(vec![0.0, 0.0, 0.0], vec![75., 75., 75.]);

    let mut kinetic_energies = [0.0; 500];
    let mut potential_energies = [0.0; 500];
    let mut index = 0;

    loop {
        window::clear_background(color::BLACK);

        let mut points = pendulum.cords.iter().peekable();
        while let Some(current) = points.next() {
            if let Some(next) = points.peek() {
                draw_line(current.x, current.y, next.x, next.y, 5.0, color::LIME);
            }
        }

        pendulum.update(time::get_frame_time());

        kinetic_energies[index] = pendulum
            .velocities
            .iter()
            .fold(0.0, |acc, v| acc + (v.length() / 100.0).powi(2))
            / 2.0;
        potential_energies[index] = pendulum.cords.iter().fold(0.0, |acc, c| {
            acc + (window::screen_height() - c.y) / 100.0 * 9.8
        }) / 2.0;
        index += 1;
        if index >= 500 {
            index = 0;
        }

        let mut energies = kinetic_energies
            .iter()
            .zip(potential_energies.iter())
            .rev()
            .enumerate()
            .peekable();
        while let Some((curr_i, (&curr_k, &curr_p))) = energies.next() {
            if let Some((next_i, (next_k, next_p))) = energies.peek() {
                if *next_i == 500 - index || curr_i == 500 - index {
                    continue;
                }
                draw_line(
                    window::screen_width() - curr_i as f32,
                    window::screen_height() - curr_k,
                    window::screen_width() - *next_i as f32,
                    window::screen_height() - **next_k,
                    1.0,
                    color::RED,
                );
                draw_line(
                    window::screen_width() - curr_i as f32,
                    window::screen_height() - curr_p,
                    window::screen_width() - *next_i as f32,
                    window::screen_height() - **next_p,
                    1.0,
                    color::BLUE,
                );
                draw_line(
                    window::screen_width() - curr_i as f32,
                    window::screen_height() - (curr_k + curr_p),
                    window::screen_width() - *next_i as f32,
                    window::screen_height() - (**next_k + **next_p),
                    1.0,
                    color::PURPLE,
                );
            }
        }

        window::next_frame().await
    }
}
