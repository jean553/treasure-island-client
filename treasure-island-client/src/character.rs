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

    /// Returns the character sprite.
    pub fn get_sprite(&self) -> &G2dTexture {
        &self.sprite
    }

    /// Returns the character position.
    pub fn get_position(&self) -> usize {
        self.position
    }
}
