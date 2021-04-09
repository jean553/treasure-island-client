//! Structure and implementation of one graphical character.

use piston_window::G2dTexture;

/// Displayable character structure.
pub struct Character {
    sprite: G2dTexture,
    position: usize,
}

impl Character {

    /// Character constructor.
    ///
    /// # Args:
    ///
    /// `sprite` - the character sprite
    /// `position` - the character default position
    pub fn new(
        sprite: G2dTexture,
        position: usize,
    ) -> Self {
        Character {
            sprite: sprite,
            position: position,
        }
    }
}
