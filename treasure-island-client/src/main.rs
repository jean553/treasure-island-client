extern crate piston_window;

mod gui;
mod sprite;

use gui::display_sprites;
use sprite::load_sprite_from_file;

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
use std::net::TcpStream;
use std::thread::spawn;
use std::io::{
    BufReader,
    Read,
};
use std::sync::{
    Mutex,
    Arc,
};

/// Contains the whole code of a dedicated thread.
/// Continuously checks for messages coming from the server.
///
/// Args:
///
/// `stream` - the stream to listen messages from
/// `tiles_mutex_arc` - thread-safe pointer to the tiles array
fn receive_message_from_stream(
    stream: TcpStream,
    tiles_mutex_arc: Arc<Mutex<[u8; 400]>>,
) {

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

        /* block until there is no use of the tiles mutex pointer anymore */
        let mut tiles_mutex_guard = tiles_mutex_arc.lock().unwrap();
        let tiles = &mut *tiles_mutex_guard;
        tiles.copy_from_slice(&message);

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

    const TILES_AMOUNT: usize = 400;
    let tiles: [u8; TILES_AMOUNT] = [0; TILES_AMOUNT];

    let tiles_mutex: Mutex<[u8; TILES_AMOUNT]> = Mutex::new(tiles);
    let tiles_mutex_arc: Arc<Mutex<[u8; TILES_AMOUNT]>> = Arc::new(tiles_mutex);
    let tiles_mutex_arc_main_thread = tiles_mutex_arc.clone();

    /* FIXME: #13 the address should be the domain name of the real server,
       only work with a local server for now;
       this part should be improved as the server has to be up
       for the client to start which is only a temporary solution */
    let stream = TcpStream::connect("127.0.0.1:9500").unwrap();

    spawn(|| {
        receive_message_from_stream(
            stream,
            tiles_mutex_arc,
        );
    });

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

                let tiles_mutex_guard = tiles_mutex_arc_main_thread.lock().unwrap();
                let tiles = &*tiles_mutex_guard;

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
