extern crate piston_window;

mod gui;
mod sprite;

use gui::display_sprites;
use sprite::{
    load_sprite_from_file,
    get_sprite_index_from_tile_value,
};

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
use std::collections::VecDeque;
use std::net::TcpStream;
use std::thread::spawn;
use std::io::{
    BufReader,
    Read,
};

use rand::thread_rng;
use rand::Rng;

/// Contains the whole code of a dedicated thread.
/// Continuously checks for messages coming from the server.
///
/// Args:
///
/// `stream` - the stream to listen messages from
fn receive_message_from_stream(stream: TcpStream) {

    let mut buffer = BufReader::new(stream);

    const BUFFER_LENGTH: usize = 400;
    let mut message: [u8; BUFFER_LENGTH] = [0; BUFFER_LENGTH];

    println!("Listening for messages from the server...");

    loop {

        /* blocking until the client receives a message from the server;
           a line break is expected at the end of the line */
        let _ = buffer.read(&mut message).unwrap();

        /* only considers received messages if the first byte is not equal to 0 */
        if message[0] == 0 {
            continue;
        }

        /* TODO: use mpsc channels to use message from the main thread */
        println!("Received message: {:?}", message);
    }
}

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

    let all_sprites = [

        /* from 0 to 3: sands */
        load_sprite_from_file(&mut window, "sand_1.png"),
        load_sprite_from_file(&mut window, "sand_2.png"),
        load_sprite_from_file(&mut window, "sand_3.png"),
        load_sprite_from_file(&mut window, "sand_4.png"),

        /* from 4 to 7: sands and water links */
        load_sprite_from_file(&mut window, "sand_water_1.png"),
        load_sprite_from_file(&mut window, "sand_water_2.png"),
        load_sprite_from_file(&mut window, "sand_water_3.png"),
        load_sprite_from_file(&mut window, "sand_water_4.png"),

        /* at 8: palm tree */
        load_sprite_from_file(&mut window, "palm_1.png"),

        /* at 9: chest */
        load_sprite_from_file(&mut window, "chest_1.png"),

        /* at 10: water */
        load_sprite_from_file(&mut window, "water_1.png"),
    ];

    /* FIXME: #13 the address should be the domain name of the real server,
       only work with a local server for now;
       this part should be improved as the server has to be up
       for the client to start which is only a temporary solution */
    let stream = TcpStream::connect("127.0.0.1:9500").unwrap();

    spawn(|| {
        receive_message_from_stream(stream);
    });

    const TILES_AMOUNT: usize = 400;
    const DEFAULT_TILE_VALUE: u8 = 0;
    let mut tiles: [u8; TILES_AMOUNT] = [
        DEFAULT_TILE_VALUE;
        TILES_AMOUNT
    ];

    const TILES_PER_LINE: usize = 20;
    const LAST_LINE_FIRST_TILE_INDEX: usize = 380;
    
    const SAND_WATER_BOTTOM_SPRITE_INDEX: u8 = 4;
    const SAND_WATER_TOP_SPRITE_INDEX: u8 = 5;
    const SAND_WATER_LEFT_SPRITE_INDEX: u8 = 6;
    const SAND_WATER_RIGHT_SPRITE_INDEX: u8 = 7;

    let mut range = thread_rng();

    let mut previous_tile_value: u8 = 0;
    let mut previous_line: VecDeque<u8> = VecDeque::new();

    for (index, tile) in tiles.iter_mut().enumerate() {

        if index < TILES_PER_LINE {
            *tile = SAND_WATER_LEFT_SPRITE_INDEX; 
            continue;
        }

        if index % TILES_PER_LINE == 0 {
            *tile = SAND_WATER_TOP_SPRITE_INDEX;
            continue;
        }

        if index % TILES_PER_LINE == TILES_PER_LINE - 1 {
            *tile = SAND_WATER_RIGHT_SPRITE_INDEX;
            continue;
        }

        if index >= LAST_LINE_FIRST_TILE_INDEX {
            *tile = SAND_WATER_BOTTOM_SPRITE_INDEX;
            continue;
        }

        const WATER_TILE_VALUE: u8 = 0;
        const TREE_TILE_VALUE: u8 = 3;

        const FIRST_ISLAND_TILE_INDEX: usize = 21;
        if index == FIRST_ISLAND_TILE_INDEX {

            let tile_value = range.gen_range(WATER_TILE_VALUE..TREE_TILE_VALUE + 1);
            *tile = get_sprite_index_from_tile_value(tile_value);

            previous_tile_value = tile_value;
            previous_line.push_back(tile_value);

            continue;
        }

        const SECOND_LINE_OF_ISLAND_TILES_FIRST_INDEX: usize = 41;
        if index >= SECOND_LINE_OF_ISLAND_TILES_FIRST_INDEX {
            previous_tile_value = previous_line.pop_front().unwrap();
        }

        let mut minimum: u8 = WATER_TILE_VALUE;
        let mut maximum: u8 = TREE_TILE_VALUE;

        if previous_tile_value != WATER_TILE_VALUE {
            minimum = previous_tile_value - 1;
        }

        if previous_tile_value != TREE_TILE_VALUE {
            maximum = previous_tile_value + 1;
        }

        let tile_value = range.gen_range(minimum..maximum + 1);
        *tile = get_sprite_index_from_tile_value(tile_value);

        if index > FIRST_ISLAND_TILE_INDEX &&
            index < SECOND_LINE_OF_ISLAND_TILES_FIRST_INDEX {
            previous_tile_value = tile_value;
        }

        previous_line.push_back(tile_value);
    }

    /* force angles to have water */
    const FIRST_MAP_ANGLE_TILE_INDEX: usize = 0;
    const SECOND_MAP_ANGLE_TILE_INDEX: usize = 19;
    const THIRD_MAP_ANGLE_TILE_INDEX: usize = 380;
    const FOURTH_MAP_ANGLE_TILE_INDEX: usize = 399;
    const WATER_TILE_SPRITE_INDEX: u8 = 10;
    tiles[FIRST_MAP_ANGLE_TILE_INDEX] = WATER_TILE_SPRITE_INDEX;
    tiles[SECOND_MAP_ANGLE_TILE_INDEX] = WATER_TILE_SPRITE_INDEX;
    tiles[THIRD_MAP_ANGLE_TILE_INDEX] = WATER_TILE_SPRITE_INDEX;
    tiles[FOURTH_MAP_ANGLE_TILE_INDEX] = WATER_TILE_SPRITE_INDEX;

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

                display_sprites(
                    window,
                    &context.transform,
                    &all_sprites,
                    &tiles,
                    origin_horizontal_position,
                    origin_vertical_position,
                );
            }
        );
    }
}
