//! Handles the main game screen.

use crate::gui::{
    display_sprites,
    display_characters,
};
use crate::sprite::load_sprite_from_file;
use crate::character::Character;

use piston_window::{
    PistonWindow,
    Context,
    G2d,
    Event,
    Key,
    Button,
    PressEvent,
    G2dTexture,
};

use std::sync::{
    Mutex,
    Arc,
};
use std::time::Instant;

const TILES_AMOUNT: usize = 400;
const SPRITES_AMOUNT: usize = 11;
const CHARACTERS_AMOUNT: usize = 2;

type Tiles = Arc<Mutex<[u8; TILES_AMOUNT]>>;

pub struct GameScreen {
    tiles: Tiles,
    characters: [Character; CHARACTERS_AMOUNT],
    sprites: [G2dTexture; SPRITES_AMOUNT],
    origin_horizontal_position: f64,
    origin_vertical_position: f64,
    event_previous_time: Instant,
}

impl GameScreen {

    /// Constructor. Loads the sprites and the characters.
    ///
    /// # Args:
    ///
    /// `window` - the Piston window to use; expected to be mutable to load all the sprites
    /// `tiles` - thread safe pointer to the tiles; shared with the thread that receives updated tiles map from the server
    pub fn new(
        window: &mut PistonWindow,
        tiles: Tiles,
    ) -> GameScreen {

        let sprites = [

            /* from 0 to 3: sands */
            load_sprite_from_file(window, "sand_1.png"),
            load_sprite_from_file(window, "sand_2.png"),
            load_sprite_from_file(window, "sand_3.png"),
            load_sprite_from_file(window, "sand_4.png"),

            /* from 4 to 7: sands and water links */
            load_sprite_from_file(window, "sand_water_1.png"),
            load_sprite_from_file(window, "sand_water_2.png"),
            load_sprite_from_file(window, "sand_water_3.png"),
            load_sprite_from_file(window, "sand_water_4.png"),

            /* at 8: palm tree */
            load_sprite_from_file(window, "palm_1.png"),

            /* at 9: chest */
            load_sprite_from_file(window, "chest_1.png"),

            /* at 10: water */
            load_sprite_from_file(window, "water_1.png"),
        ];

        /* FIXME: we use fixed default positions for our characters for now;
           that information should come from the server */
        const FIRST_CHARACTER_DEFAULT_POSITION: usize = 38;
        const SECOND_CHARACTER_DEFAULT_POSITION: usize = 361;
        let characters: [Character; CHARACTERS_AMOUNT] = [
            Character::new(
                load_sprite_from_file(window, "character_1.png"),
                FIRST_CHARACTER_DEFAULT_POSITION
            ),
            Character::new(
                load_sprite_from_file(window, "character_2.png"),
                SECOND_CHARACTER_DEFAULT_POSITION
            ),
        ];

        GameScreen {
            tiles: tiles,
            characters: characters,
            sprites: sprites,
            origin_horizontal_position: 0.0,
            origin_vertical_position: 0.0,
            event_previous_time: Instant::now(),
        }
    }

    /// Renders the screen.
    ///
    /// # Args:
    ///
    /// `context` - the Piston context to use
    /// `window` - the Piston window to use expected to be mutable to display stuffs on screen
    pub fn render(
        &self,
        context: Context,
        window: &mut G2d,
    ) {
        let tiles_mutex_guard = self.tiles.lock().unwrap();
        let tiles = &*tiles_mutex_guard;

        display_sprites(
            window,
            &context.transform,
            &self.sprites,
            &tiles,
            self.origin_horizontal_position,
            self.origin_vertical_position,
        );

        display_characters(
            window,
            &context.transform,
            &self.characters,
            self.origin_horizontal_position,
            self.origin_vertical_position,
        );
    }

    /// Handle the events of the screen. Mutable as it modifies the camera position.
    ///
    /// # Args:
    ///
    /// `event` - the event to handle
    pub fn handle_events(
        &mut self,
        event: &Event
    ) {

        let pressed_key = event.press_args();

        const CAMERA_MOVEMENT_OFFSET: f64 = 10.0;
        const CAMERA_MOVEMENT_INTERVAL: u128 = 25;

        if let Some(Button::Keyboard(Key::Up)) = pressed_key {
            if Instant::now().duration_since(self.event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                    self.origin_vertical_position += CAMERA_MOVEMENT_OFFSET;
                    self.event_previous_time = Instant::now();
                }
        }
        else if let Some(Button::Keyboard(Key::Down)) = pressed_key {
            if Instant::now().duration_since(self.event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                    self.origin_vertical_position -= CAMERA_MOVEMENT_OFFSET;
                    self.event_previous_time = Instant::now();
                }
        }
        else if let Some(Button::Keyboard(Key::Left)) = pressed_key {
            if Instant::now().duration_since(self.event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                    self.origin_horizontal_position += CAMERA_MOVEMENT_OFFSET;
                    self.event_previous_time = Instant::now();
                }
        }
        else if let Some(Button::Keyboard(Key::Right)) = pressed_key {
            if Instant::now().duration_since(self.event_previous_time).as_millis() >
                CAMERA_MOVEMENT_INTERVAL {
                    self.origin_horizontal_position -= CAMERA_MOVEMENT_OFFSET;
                    self.event_previous_time = Instant::now();
                }
        }
    }
}
