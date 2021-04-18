use piston_window::text::Text;
use piston_window::{
    Glyphs,
    Context,
    Transformed,
    G2d,
    Event,
    Key,
    Button,
    PressEvent,
};

use gfx_device_gl::Device;

pub struct UsernamePromptScreen {
    username: String,
}

impl UsernamePromptScreen {

    /// Constructor.
    pub fn new() -> UsernamePromptScreen {

        const DEFAULT_USERNAME: &str = "";
        UsernamePromptScreen {
            username: DEFAULT_USERNAME.to_string()
        }
    }

    /// Renders the screen.
    ///
    /// # Args:
    ///
    /// `context` - the Piston context to use
    /// `window` - the Piston window to use
    /// `device` - the Piston device to use
    /// `font` - the font to use to render characters
    pub fn render(
        &self,
        context: Context,
        window: &mut G2d,
        device: &mut Device,
        font: &mut Glyphs,
    ) {

        const WHITE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        const CHOOSE_YOUR_USERNAME_MESSAGE_FONT_SIZE: u32 = 64;
        const CHOOSE_YOUR_USERNAME_MESSAGE_HORIZONTAL_POSITION: f64 = 635.0;
        const CHOOSE_YOUR_USERNAME_MESSAGE_VERTICAL_POSITION: f64 = 500.0;
        const CHOOSE_YOUR_USERNAME_MESSAGE: &str = "Choose your username:";
        Text::new_color(
            WHITE_COLOR,
            CHOOSE_YOUR_USERNAME_MESSAGE_FONT_SIZE,
        ).draw(
            CHOOSE_YOUR_USERNAME_MESSAGE,
            font,
            &context.draw_state,
            context.transform.trans(
                CHOOSE_YOUR_USERNAME_MESSAGE_HORIZONTAL_POSITION,
                CHOOSE_YOUR_USERNAME_MESSAGE_VERTICAL_POSITION,
            ),
            window
        ).unwrap();

        const USERNAME_MESSAGE_FONT_SIZE: u32 = 64;
        const USERNAME_MESSAGE_HORIZONTAL_POSITION: f64 = 635.0;
        const USERNAME_MESSAGE_VERTICAL_POSITION: f64 = 600.0;
        Text::new_color(
            WHITE_COLOR,
            USERNAME_MESSAGE_FONT_SIZE,
        ).draw(
            &self.username,
            font,
            &context.draw_state,
            context.transform.trans(
                USERNAME_MESSAGE_HORIZONTAL_POSITION,
                USERNAME_MESSAGE_VERTICAL_POSITION,
            ),
            window
        ).unwrap();

        font.factory
            .encoder
            .flush(device);
    }

    /// Handle the events of the screen.
    ///
    /// # Args:
    ///
    /// `event` - the event to handle
    pub fn handle_events(
        &mut self,
        event: &Event
    ) {

        let pressed_key = event.press_args();

        if let Some(Button::Keyboard(Key::A)) = pressed_key {
            const CHARACTER: &str = "A";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::B)) = pressed_key {
            const CHARACTER: &str = "B";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::C)) = pressed_key {
            const CHARACTER: &str = "C";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::D)) = pressed_key {
            const CHARACTER: &str = "D";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::E)) = pressed_key {
            const CHARACTER: &str = "E";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::F)) = pressed_key {
            const CHARACTER: &str = "F";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::G)) = pressed_key {
            const CHARACTER: &str = "G";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::H)) = pressed_key {
            const CHARACTER: &str = "H";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::I)) = pressed_key {
            const CHARACTER: &str = "I";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::J)) = pressed_key {
            const CHARACTER: &str = "J";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::K)) = pressed_key {
            const CHARACTER: &str = "K";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::L)) = pressed_key {
            const CHARACTER: &str = "L";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::M)) = pressed_key {
            const CHARACTER: &str = "M";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::N)) = pressed_key {
            const CHARACTER: &str = "N";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::O)) = pressed_key {
            const CHARACTER: &str = "O";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::P)) = pressed_key {
            const CHARACTER: &str = "P";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::Q)) = pressed_key {
            const CHARACTER: &str = "Q";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::R)) = pressed_key {
            const CHARACTER: &str = "R";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::S)) = pressed_key {
            const CHARACTER: &str = "S";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::T)) = pressed_key {
            const CHARACTER: &str = "T";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::U)) = pressed_key {
            const CHARACTER: &str = "U";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::V)) = pressed_key {
            const CHARACTER: &str = "V";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::W)) = pressed_key {
            const CHARACTER: &str = "W";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::X)) = pressed_key {
            const CHARACTER: &str = "X";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::Y)) = pressed_key {
            const CHARACTER: &str = "Y";
            self.username.push_str(CHARACTER);
        }
        else if let Some(Button::Keyboard(Key::Z)) = pressed_key {
            const CHARACTER: &str = "Z";
            self.username.push_str(CHARACTER);
        }
    }
}
