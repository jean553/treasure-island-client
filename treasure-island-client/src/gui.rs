//! Handles GUI actions.

use piston_window::{
    G2d,
    G2dTexture,
    Transformed,
    rectangle,
    image,
};

/// Display the tiles. Called only once but refactored into a function for readability. Optimized
/// to not render the tiles if outside of the camera viewport.
///
/// # Args:
///
/// `window` - the window where the selector is displayed
/// `transform` - transformation to apply on the selector when drawing
/// `all_sprites` - the list of all availables sprites
/// `tiles` - the displayed tiles list
/// `origin_horizontal_position` - the origin horizontal position
/// `origin_vertical_position` - the origin vertical position
pub fn display_tiles(
    window: &mut G2d,
    transform: &[[f64; 3]; 2],
    all_sprites: &[G2dTexture],
    tiles: &[usize; 400],
    origin_horizontal_position: f64,
    origin_vertical_position: f64,
) {

    let mut column: usize = 0;
    let mut line: usize = 0;

    for (index, tile) in tiles.iter().enumerate() {

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

        const TILE_HORIZONTAL_DISTANCE: f64 = 70.0;
        let tile_horizontal_position = TILE_HORIZONTAL_OFFSET -
            TILE_HORIZONTAL_DISTANCE * (column as f64) +
            TILE_HORIZONTAL_DISTANCE * (line as f64) +
            origin_horizontal_position;

        const TILE_WIDTH: f64 = 140.0;
        if tile_horizontal_position < -TILE_WIDTH ||
            tile_horizontal_position > WINDOW_WIDTH {
            column += 1;
            continue;
        }

        const TILE_VERTICAL_DISTANCE: f64 = 32.0;
        let tile_vertical_position = TILE_VERTICAL_OFFSET +
            TILE_VERTICAL_DISTANCE * (column as f64) +
            TILE_VERTICAL_DISTANCE * (line as f64) +
            origin_vertical_position;

        const TILE_HEIGHT: f64 = 140.0;
        if tile_vertical_position < -TILE_HEIGHT ||
            tile_vertical_position > WINDOW_HEIGHT {
            column += 1;
            continue;
        }

        image(
            &all_sprites[*tile],
            transform.trans(
                tile_horizontal_position,
                tile_vertical_position
            ),
            window,
        );

        column += 1;
    }
}
