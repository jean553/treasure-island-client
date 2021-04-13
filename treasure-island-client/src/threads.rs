//! Contains individual threads dedicated codes.

use crate::screen::Screen;

use std::net::TcpStream;
use std::sync::{
    Mutex,
    Arc,
};
use std::io::{
    BufReader,
    Read,
};

/// Contains the whole code of a dedicated thread.
/// Continuously checks for messages coming from the server.
///
/// Args:
///
/// `stream` - the stream to listen messages from
/// `tiles_mutex_arc` - thread-safe pointer to the tiles array
/// `current_screen_mutex_arc` - thread-safe pointer on the currently displayed screen
pub fn receive_message_from_stream(
    stream: TcpStream,
    tiles_mutex_arc: Arc<Mutex<[u8; 400]>>,
    current_screen_mutex_arc: Arc<Mutex<Screen>>,
) {

    let mut buffer = BufReader::new(stream);

    /* maximum data received length is 400 bytes long (for the map);
       + 1 byte for the action */
    const BUFFER_LENGTH: usize = 401;
    let mut message: [u8; BUFFER_LENGTH] = [0; BUFFER_LENGTH];

    println!("Listening for messages from the server...");

    loop {

        /* blocking */
        let _ = buffer.read(&mut message).unwrap();

        let message_action = message[0];

        const MESSAGE_ACTION_IGNORED: u8 = 0;
        if message_action == MESSAGE_ACTION_IGNORED {
            continue;
        }

        const MESSAGE_ACTION_PUSH_MAP: u8 = 1;
        if message_action == MESSAGE_ACTION_PUSH_MAP {

            let mut tiles_mutex_guard = tiles_mutex_arc.lock().unwrap();
            let tiles = &mut *tiles_mutex_guard;
            tiles.copy_from_slice(&message[1..BUFFER_LENGTH]);

            continue;
        }

        let mut current_screen_mutex_guard = current_screen_mutex_arc.lock().unwrap();
        let current_screen = &mut *current_screen_mutex_guard;
        *current_screen = Screen::Game;
    }
}

