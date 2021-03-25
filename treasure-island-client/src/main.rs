extern crate piston_window;

mod gui;
mod load_texture;

use gui::display_tiles;
use load_texture::load_texture_from_file;

use piston_window::{
    PistonWindow,
    WindowSettings,
    clear,
    PressEvent,
    Button,
    Key,
};

use piston_window::color::hex;

use std::time;

use rand::thread_rng;
use rand::Rng;

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

    let all_tiles = [

        /* from 0 to 3: sands */
        load_texture_from_file(&mut window, "sand_1.png"),
        load_texture_from_file(&mut window, "sand_2.png"),
        load_texture_from_file(&mut window, "sand_3.png"),
        load_texture_from_file(&mut window, "sand_4.png"),

        /* from 4 to 7: sands and water links */
        load_texture_from_file(&mut window, "sand_water_1.png"),
        load_texture_from_file(&mut window, "sand_water_2.png"),
        load_texture_from_file(&mut window, "sand_water_3.png"),
        load_texture_from_file(&mut window, "sand_water_4.png"),

        /* at 8: palm tree */
        load_texture_from_file(&mut window, "palm_1.png"),

        /* at 9: chest */
        load_texture_from_file(&mut window, "chest_1.png"),

        /* at 10: water */
        load_texture_from_file(&mut window, "water_1.png"),
    ];

    const TILES_AMOUNT: usize = 400;
    const DEFAULT_TILE_INDEX: u8 = 0;
    let mut tiles: [u8; TILES_AMOUNT] = [
        DEFAULT_TILE_INDEX;
        TILES_AMOUNT
    ];

    const TILES_PER_LINE: usize = 20;
    const LAST_LINE_FIRST_TILE_INDEX: usize = 380;
    
    const SAND_WATER_BOTTOM_INDEX: u8 = 4;
    const SAND_WATER_TOP_INDEX: u8 = 5;
    const SAND_WATER_LEFT_INDEX: u8 = 6;
    const SAND_WATER_RIGHT_INDEX: u8 = 7;

    /* FIXME: only generate random sand tiles for now,
       should generate noise-type array with sand, trees... etc... */
    let mut range = thread_rng();

    for (index, tile) in tiles.iter_mut().enumerate() {

        if index < TILES_PER_LINE {
            *tile = SAND_WATER_LEFT_INDEX; 
            continue;
        }

        if index % TILES_PER_LINE == 0 {
            *tile = SAND_WATER_TOP_INDEX;
            continue;
        }

        if index % TILES_PER_LINE == TILES_PER_LINE - 1 {
            *tile = SAND_WATER_RIGHT_INDEX;
            continue;
        }

        if index >= LAST_LINE_FIRST_TILE_INDEX {
            *tile = SAND_WATER_BOTTOM_INDEX;
            continue;
        }

        const FIRST_SAND_TILE_INDEX: u8 = 0;
        const SAND_TILES_AMOUNT: u8 = 4;
        *tile = range.gen_range(FIRST_SAND_TILE_INDEX..SAND_TILES_AMOUNT);
    }

    /* force angles to have water */
    const FIRST_MAP_ANGLE_INDEX: usize = 0;
    const SECOND_MAP_ANGLE_INDEX: usize = 19;
    const THIRD_MAP_ANGLE_INDEX: usize = 380;
    const FOURTH_MAP_ANGLE_INDEX: usize = 399;
    const WATER_TILE_INDEX: u8 = 10;
    tiles[FIRST_MAP_ANGLE_INDEX] = WATER_TILE_INDEX;
    tiles[SECOND_MAP_ANGLE_INDEX] = WATER_TILE_INDEX;
    tiles[THIRD_MAP_ANGLE_INDEX] = WATER_TILE_INDEX;
    tiles[FOURTH_MAP_ANGLE_INDEX] = WATER_TILE_INDEX;

    let mut origin_horizontal_position: f64 = 0.0;
    let mut origin_vertical_position: f64 = 0.0;

    let mut event_previous_time = time::Instant::now();

    while let Some(event) = window.next() {

        let pressed_key = event.press_args();

        const CAMERA_MOVEMENT_OFFSET: f64 = 10.0;
        const CAMERA_MOVEMENT_INTERVAL: u128 = 25;

        if let Some(Button::Keyboard(Key::Up)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_vertical_position += CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Down)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_vertical_position -= CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Left)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_horizontal_position += CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Right)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_horizontal_position -= CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }

        window.draw_2d(
            &event,
            |context, window, _| {

                const BACKGROUND_COLOR: &str = "88FFFF"; /* light blue */
                clear(hex(BACKGROUND_COLOR), window);

                display_tiles(
                    window,
                    &context.transform,
                    &all_tiles,
                    &tiles,
                    origin_horizontal_position,
                    origin_vertical_position,
                );
            }
        );
    }
}
