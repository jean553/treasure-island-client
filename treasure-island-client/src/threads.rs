//! Contains individual threads dedicated codes.

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
/// `waiting_for_players_mutex_arc` - thread-safe pointer on boolean to be triggered when enough users have joined the game
pub fn receive_message_from_stream(
    stream: TcpStream,
    tiles_mutex_arc: Arc<Mutex<[u8; 400]>>,
    waiting_for_players_mutex_arc: Arc<Mutex<bool>>,
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

        /* FIXME: we start the game immediatly when we receive the first message
           from the server; for now, we update that value for every received message;
           we should update it only for some type of "StartGame" message */
        let mut waiting_for_players_mutex_guard = waiting_for_players_mutex_arc.lock().unwrap();
        let waiting_for_players = &mut *waiting_for_players_mutex_guard;
        *waiting_for_players = false;

        /* block until there is no use of the tiles mutex pointer anymore */
        let mut tiles_mutex_guard = tiles_mutex_arc.lock().unwrap();
        let tiles = &mut *tiles_mutex_guard;
        tiles.copy_from_slice(&message);

        /* TODO: use mpsc channels to use message from the main thread */
        println!("Received message: {:?}", message);
    }
}

