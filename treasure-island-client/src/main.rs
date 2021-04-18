extern crate piston_window;
extern crate gfx_device_gl;

mod gui;
mod sprite;
mod character;
mod threads;
mod screen;

mod username_prompt_screen;
mod waiting_for_players_screen;

use gui::{
    display_sprites,
    display_characters,
};
use sprite::load_sprite_from_file;
use character::Character;
use threads::receive_message_from_stream;
use screen::Screen;

use username_prompt_screen::UsernamePromptScreen;
use waiting_for_players_screen::WaitingForPlayersScreen;

use piston_window::{
    PistonWindow,
    WindowSettings,
    clear,
    PressEvent,
    Button,
    Key,
    TextureSettings,
    Glyphs,
};

use piston_window::color::hex;

use std::time;
use std::net::TcpStream;
use std::thread::spawn;
use std::sync::{
    Mutex,
    Arc,
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

    const WAITING_FOR_PLAYERS_MESSAGE_FONT_FILE_PATH: &str = "res/fonts/pirates-writers.ttf";
    let mut font = Glyphs::new(
        WAITING_FOR_PLAYERS_MESSAGE_FONT_FILE_PATH,
        window.create_texture_context(),
        TextureSettings::new(),
    ).unwrap();

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

    const CHARACTERS_AMOUNT: usize = 2;

    /* FIXME: we use fixed default positions for our characters for now;
       that information should come from the server */
    const FIRST_CHARACTER_DEFAULT_POSITION: usize = 38;
    const SECOND_CHARACTER_DEFAULT_POSITION: usize = 361;
    let all_characters: [Character; CHARACTERS_AMOUNT] = [
        Character::new(
            load_sprite_from_file(&mut window, "character_1.png"),
            FIRST_CHARACTER_DEFAULT_POSITION
        ),
        Character::new(
            load_sprite_from_file(&mut window, "character_2.png"),
            SECOND_CHARACTER_DEFAULT_POSITION
        ),
    ];

    const TILES_AMOUNT: usize = 400;
    let tiles: [u8; TILES_AMOUNT] = [0; TILES_AMOUNT];

    let tiles_mutex: Mutex<[u8; TILES_AMOUNT]> = Mutex::new(tiles);
    let tiles_mutex_arc: Arc<Mutex<[u8; TILES_AMOUNT]>> = Arc::new(tiles_mutex);
    let tiles_mutex_arc_main_thread = tiles_mutex_arc.clone();

    let current_screen: Screen = Screen::UsernamePrompt;
    let current_screen_mutex: Mutex<Screen> = Mutex::new(current_screen);
    let current_screen_mutex_arc: Arc<Mutex<Screen>> = Arc::new(current_screen_mutex);
    let current_screen_mutex_arc_main_thread = current_screen_mutex_arc.clone();

    /* FIXME: #13 the address should be the domain name of the real server,
       only work with a local server for now;
       this part should be improved as the server has to be up
       for the client to start which is only a temporary solution */
    let stream = TcpStream::connect("127.0.0.1:9500").unwrap();

    spawn(|| {
        receive_message_from_stream(
            stream,
            tiles_mutex_arc,
            current_screen_mutex_arc,
        );
    });

    let mut origin_horizontal_position: f64 = 0.0;
    let mut origin_vertical_position: f64 = 0.0;

    let mut event_previous_time = time::Instant::now();

    let mut username_prompt_screen = UsernamePromptScreen::new();
    let waiting_for_players_screen = WaitingForPlayersScreen::new();

    while let Some(event) = window.next() {

        if current_screen == Screen::UsernamePrompt {
            username_prompt_screen.handle_events(&event);
        }

        else if current_screen == Screen::Game {

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
        }

        window.draw_2d(
            &event,
            |context, window, device| {

                const BACKGROUND_COLOR: &str = "88FFFF"; /* light blue */
                clear(hex(BACKGROUND_COLOR), window);

                let current_screen_mutex_guard = current_screen_mutex_arc_main_thread.lock().unwrap();
                let current_screen_guard = &*current_screen_mutex_guard;
                let current_screen = *current_screen_guard;

                if current_screen == Screen::UsernamePrompt {

                    username_prompt_screen.render(
                        context,
                        window,
                        device,
                        &mut font,
                    );

                    return;
                }

                if current_screen == Screen::WaitingForPlayers {

                    waiting_for_players_screen.render(
                        context,
                        window,
                        device,
                        &mut font,
                    );

                    return;
                }

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

                display_characters(
                    window,
                    &context.transform,
                    &all_characters,
                    origin_horizontal_position,
                    origin_vertical_position,
                );
            }
        );
    }
}
