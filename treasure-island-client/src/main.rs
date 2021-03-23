extern crate piston_window;

mod gui;

use piston_window::{
    PistonWindow,
    WindowSettings,
    G2dTexture,
    Texture,
    TextureSettings,
    Flip,
    clear,
    PressEvent,
    Button,
    Key,
};

use piston_window::color::hex;

use gui::display_tiles;

use std::time;

use rand::thread_rng;
use rand::Rng;

/// Refactored code to load a texture from a given image file name. Looks for files into the images resources folder.
///
/// # Args:
///
/// `window` - the window where the textures will be displayed
/// `image` - the file of the image to load
fn load_texture_from_file(
    window: &mut PistonWindow,
    file_name: &str,
) -> G2dTexture {

    const IMAGES_FOLDER: &str = "res/images/";
    let file_path = format!(
        "{}/{}",
        IMAGES_FOLDER,
        file_name,
    );

    Texture::from_path(
        &mut window.create_texture_context(),
        file_path,
        Flip::None,
        &TextureSettings::new(),
    ).unwrap()
}

fn main() {

    const WINDOW_WIDTH: f64 = 1920.0;
    const WINDOW_HEIGHT: f64 = 1080.0;

    let mut window: PistonWindow = WindowSettings::new(
        "Treasure Island",
        [
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ],
    )
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let all_tiles = [
        load_texture_from_file(&mut window, "sand_1.png"),
        load_texture_from_file(&mut window, "sand_2.png"),
        load_texture_from_file(&mut window, "sand_3.png"),
        load_texture_from_file(&mut window, "sand_4.png"),
        load_texture_from_file(&mut window, "sand_water_1.png"),
        load_texture_from_file(&mut window, "sand_water_2.png"),
        load_texture_from_file(&mut window, "sand_water_3.png"),
        load_texture_from_file(&mut window, "sand_water_4.png"),
    ];

    const TILES_AMOUNT: usize = 400;
    const DEFAULT_TILE_INDEX: usize = 0;
    let mut tiles: [usize; TILES_AMOUNT] = [
        DEFAULT_TILE_INDEX;
        TILES_AMOUNT
    ];

    const TILES_PER_LINE: usize = 20;
    const LAST_LINE_FIRST_TILE_INDEX: usize = 380;
    
    const SAND_WATER_BOTTOM_INDEX: usize = 4;
    const SAND_WATER_TOP_INDEX: usize = 5;
    const SAND_WATER_LEFT_INDEX: usize = 6;
    const SAND_WATER_RIGHT_INDEX: usize = 7;

    /* FIXME: only generate random sand tiles for now,
       should generate noise-type array with sand, trees... etc... */
    let mut range = thread_rng();

    for (index, tile) in tiles.iter_mut().enumerate() {

        if index < TILES_PER_LINE {
            *tile = SAND_WATER_LEFT_INDEX; 
            continue;
        }

        if index % TILES_PER_LINE == 0 {
            *tile = SAND_WATER_TOP_INDEX;
            continue;
        }

        if index % TILES_PER_LINE == TILES_PER_LINE - 1 {
            *tile = SAND_WATER_RIGHT_INDEX;
            continue;
        }

        if index >= LAST_LINE_FIRST_TILE_INDEX {
            *tile = SAND_WATER_BOTTOM_INDEX;
            continue;
        }

        *tile = range.gen_range(0..4) as usize;
    }

    /* FIXME: to be removed, only for tests purposes */
    tiles[380] = 4;
    tiles[381] = 4;
    tiles[382] = 4;

    let mut origin_horizontal_position: f64 = 0.0;
    let mut origin_vertical_position: f64 = 0.0;

    let mut event_previous_time = time::Instant::now();

    while let Some(event) = window.next() {

        let pressed_key = event.press_args();

        const CAMERA_MOVEMENT_OFFSET: f64 = 10.0;
        const CAMERA_MOVEMENT_INTERVAL: u128 = 25;

        if let Some(Button::Keyboard(Key::Up)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_vertical_position += CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Down)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_vertical_position -= CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Left)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_horizontal_position += CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }
        else if let Some(Button::Keyboard(Key::Right)) = pressed_key {
            if time::Instant::now().duration_since(event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                origin_horizontal_position -= CAMERA_MOVEMENT_OFFSET;
                event_previous_time = time::Instant::now();
            }
        }

        window.draw_2d(
            &event,
            |context, window, _device| {

                const BACKGROUND_COLOR: &str = "88FFFF"; /* light blue */
                clear(hex(BACKGROUND_COLOR), window);

                display_tiles(
                    window,
                    &context.transform,
                    &all_tiles,
                    &tiles,
                    origin_horizontal_position,
                    origin_vertical_position,
                );
            }
        );
    }
}
