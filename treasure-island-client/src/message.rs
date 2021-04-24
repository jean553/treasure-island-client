//! Contains the Message structure, sent through streams between server and client.

use serde_derive::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Message {

    /* we do not use enums to send actions because:
       - this is a "raw network" information to be handled both on client and server,
         having an enumeration may require to maintain same version of the structure
         from both side,
       - enumerations might take a padded space in memory according to the architecture,
         but we have to be sure we only uses one bit,
       - enumerations are tagged unions, so they may take a few bytes for the type too */
    action: u8,
    data: [u8; 32]
}

impl Message {

    /// Constructor
    ///
    /// # Args:
    ///
    /// `action` - the action of the message
    pub fn new(action: u8) -> Self {
        Message {
            action: action,
            data: [0; 32]
        }
    }

    /// Sets the data to send (32 bytes).
    ///
    /// # Args:
    ///
    /// `data` - the data to send
    pub fn set_data(
        &mut self,
        data: [u8; 32],
    ) {
        self.data = data;
    }
}
