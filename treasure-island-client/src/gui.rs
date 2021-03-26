//! Handles GUI actions.

use piston_window::{
    G2d,
    G2dTexture,
    Transformed,
    image,
};

/// Display the sprites. Called only once but refactored into a function for readability. Optimized
/// to not render the sprites if outside of the camera viewport.
///
/// # Args:
///
/// `window` - the window where the selector is displayed
/// `transform` - transformation to apply on the selector when drawing
/// `all_sprites` - the list of all availables sprites
/// `sprites` - the displayed sprites list
/// `origin_horizontal_position` - the origin horizontal position
/// `origin_vertical_position` - the origin vertical position
pub fn display_sprites(
    window: &mut G2d,
    transform: &[[f64; 3]; 2],
    all_sprites: &[G2dTexture],
    sprites: &[u8; 400],
    origin_horizontal_position: f64,
    origin_vertical_position: f64,
) {

    let mut column: usize = 0;
    let mut line: usize = 0;

    for (index, tile) in sprites.iter().enumerate() {

        const TILES_PER_LINE: usize = 20;

        if index != 0 &&
            index % TILES_PER_LINE == 0 {
            column = 0;
            line += 1;
        }

        const WINDOW_WIDTH: f64 = 1920.0;
        const WINDOW_HEIGHT: f64 = 1080.0;

        const TILE_HORIZONTAL_OFFSET: f64 = -75.0;
        const TILE_VERTICAL_OFFSET: f64 = -25.0;

        const TILE_HORIZONTAL_DISTANCE: f64 = 69.0;
        let sprite_horizontal_position = TILE_HORIZONTAL_OFFSET -
            TILE_HORIZONTAL_DISTANCE * (column as f64) +
            TILE_HORIZONTAL_DISTANCE * (line as f64) +
            origin_horizontal_position;

        const TILE_WIDTH: f64 = 140.0;
        if sprite_horizontal_position < -TILE_WIDTH ||
            sprite_horizontal_position > WINDOW_WIDTH {
            column += 1;
            continue;
        }

        const TILE_VERTICAL_DISTANCE: f64 = 31.0;
        let sprite_vertical_position = TILE_VERTICAL_OFFSET +
            TILE_VERTICAL_DISTANCE * (column as f64) +
            TILE_VERTICAL_DISTANCE * (line as f64) +
            origin_vertical_position;

        const TILE_HEIGHT: f64 = 140.0;
        if sprite_vertical_position < -TILE_HEIGHT ||
            sprite_vertical_position > WINDOW_HEIGHT {
            column += 1;
            continue;
        }

        /* sprites are all identified by a u8 number,
           as we do not want to store arrays of usize types
           more used to index others arrays;
           but here, we have to retrieve the tile image
           from the sprites array based on its index;
           so we are required to convert it as usize only here */
        let sprite_index = *tile as usize;

        image(
            &all_sprites[sprite_index],
            transform.trans(
                sprite_horizontal_position,
                sprite_vertical_position
            ),
            window,
        );

        column += 1;
    }
}
