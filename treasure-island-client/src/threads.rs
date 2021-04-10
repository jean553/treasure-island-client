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
pub fn receive_message_from_stream(
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

