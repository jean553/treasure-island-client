//! Enumeration that represents every different screen of the game.

#[derive(PartialEq, Copy, Clone)]
pub enum Screen {
    UsernamePrompt,
    WaitingForPlayers,
    Game,
}
