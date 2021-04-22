extern crate piston_window;
extern crate gfx_device_gl;

mod gui;
mod sprite;
mod character;
mod threads;
mod screen;

mod username_prompt_screen;
mod waiting_for_players_screen;
mod game_screen;

use threads::receive_message_from_stream;
use screen::Screen;

use username_prompt_screen::UsernamePromptScreen;
use waiting_for_players_screen::WaitingForPlayersScreen;
use game_screen::GameScreen;

use piston_window::{
    PistonWindow,
    WindowSettings,
    clear,
    TextureSettings,
    Glyphs,
};

use piston_window::color::hex;

use std::net::TcpStream;
use std::thread::spawn;
use std::sync::{
    Mutex,
    Arc,
};
use std::io::{
    BufReader,
    LineWriter,
};

fn main() {


    /* load global resources used everywhere (window, fonts) */

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


    /* load cross-threads resources with their pointers and mutexes
       (current screen, map) */

    const TILES_AMOUNT: usize = 400;
    let tiles: [u8; TILES_AMOUNT] = [0; TILES_AMOUNT];
    let tiles_mutex: Mutex<[u8; TILES_AMOUNT]> = Mutex::new(tiles);
    let tiles_mutex_arc: Arc<Mutex<[u8; TILES_AMOUNT]>> = Arc::new(tiles_mutex);

    let current_screen: Screen = Screen::UsernamePrompt;
    let current_screen_mutex: Mutex<Screen> = Mutex::new(current_screen);
    let current_screen_mutex_arc: Arc<Mutex<Screen>> = Arc::new(current_screen_mutex);

    /* FIXME: #13 the address should be the domain name of the real server,
       only work with a local server for now;
       this part should be improved as the server has to be up
       for the client to start which is only a temporary solution */
    let read_stream = TcpStream::connect("127.0.0.1:9500").unwrap();
    let write_stream = read_stream.try_clone().unwrap();

    let read_buffer = BufReader::new(read_stream);
    let _line_writer = LineWriter::new(write_stream);

    let tiles_mutex_arc_receive_message_thread = tiles_mutex_arc.clone();
    let current_screen_mutex_arc_receive_message_thread = current_screen_mutex_arc.clone();
    spawn(|| {
        receive_message_from_stream(
            read_buffer,
            tiles_mutex_arc_receive_message_thread,
            current_screen_mutex_arc_receive_message_thread,
        );
    });


    /* load all screens and clone their thread-safe shared resources (if any) */

    let mut username_prompt_screen = UsernamePromptScreen::new();

    let waiting_for_players_screen = WaitingForPlayersScreen::new();

    let mut game_screen = GameScreen::new(
        &mut window,
        tiles_mutex_arc,
    );

    while let Some(event) = window.next() {

        let mut current_screen_mutex_guard = current_screen_mutex_arc.lock().unwrap();
        let current_screen_guard = &mut *current_screen_mutex_guard;
        let current_screen = *current_screen_guard;

        match current_screen {
            Screen::UsernamePrompt => {
                username_prompt_screen.handle_events(
                    &event,
                    current_screen_guard,
                );
            },
            Screen::Game => {
                game_screen.handle_events(&event);
            },
            _ => {}
        };

        window.draw_2d(
            &event,
            |context, window, device| {

                const BACKGROUND_COLOR: &str = "88FFFF"; /* light blue */
                clear(hex(BACKGROUND_COLOR), window);

                match current_screen {
                    Screen::UsernamePrompt => {
                        username_prompt_screen.render(
                            context,
                            window,
                            device,
                            &mut font,
                        );
                    },
                    Screen::WaitingForPlayers => {
                        waiting_for_players_screen.render(
                            context,
                            window,
                            device,
                            &mut font,
                        );
                    },
                    Screen::Game => {
                        game_screen.render(
                            context,
                            window,
                        );
                    }
                };
            }
        );
    }
}
