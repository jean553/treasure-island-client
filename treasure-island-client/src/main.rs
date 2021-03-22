extern crate piston_window;

use piston_window::{
    PistonWindow,
    WindowSettings,
};

fn main() {

    const WINDOW_WIDTH: f64 = 1920.0;
    const WINDOW_HEIGHT: f64 = 1080.0;

    let mut window: PistonWindow = WindowSettings::new(
        "Treasure Island",
        [
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ],
    )
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, window, device| {
            }
        );
    }
}
