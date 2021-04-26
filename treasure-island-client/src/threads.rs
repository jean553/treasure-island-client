//! Contains individual threads dedicated codes.

use crate::screen::Screen;
use crate::message::Message;

use std::sync::{
    Mutex,
    Arc,
};
use std::sync::mpsc::Receiver;
use std::io::{
    BufReader,
    Read,
    Write,
};
use std::net::TcpStream;

/// Contains the whole code of a dedicated thread.
/// Continuously checks for messages coming from the server.
///
/// Args:
///
/// `buffer` - buffer linked to a stream to read the received data
/// `tiles_mutex_arc` - thread-safe pointer to the tiles array
/// `current_screen_mutex_arc` - thread-safe pointer on the currently displayed screen
pub fn receive_message_from_stream(
    mut buffer: BufReader<TcpStream>,
    tiles_mutex_arc: Arc<Mutex<[u8; 400]>>,
    current_screen_mutex_arc: Arc<Mutex<Screen>>,
) {

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

/// Contains the whole code of a dedicated thread.
/// Continuously checks for messages to be sent to the server and sends them.
///
/// Args:
///
/// `stream` - the stream connected to the server, used to send messages
/// `receiver` - receives messages sent from sender from the different screens
pub fn send_message_to_stream(
    mut stream: TcpStream,
    receiver: Receiver<Message>,
) {

    loop {

        let message = receiver.recv().unwrap();

        let data: Vec<u8> = bincode::serialize(&message).unwrap();
        stream.write(&data).unwrap();
    }
}
