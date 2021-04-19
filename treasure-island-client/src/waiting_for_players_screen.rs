//! Handles the screen that indicates the game is waiting for players.

use piston_window::text::Text;
use piston_window::{
    Glyphs,
    Context,
    Transformed,
    G2d,
};

use gfx_device_gl::Device;

pub struct WaitingForPlayersScreen {

    /* there is nothing to store into the screen*/
}

impl WaitingForPlayersScreen {

    pub fn new() -> WaitingForPlayersScreen {
        WaitingForPlayersScreen {}
    }

    /// Renders the screen.
    ///
    /// # Args:
    ///
    /// `context` - the Piston context to use
    /// `window` - the Piston window to use expected to be mutable to display stuffs on screen
    /// `device` - the Piston device to use; expected to be mutable to display stuffs on screen
    /// `font` - the font to use to render characters; expected to be mutable to render text
    pub fn render(
        &self,
        context: Context,
        window: &mut G2d,
        device: &mut Device,
        font: &mut Glyphs,
    ) {

        const WHITE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const WAITING_FOR_PLAYERS_MESSAGE_FONT_SIZE: u32 = 64;
        const WAITING_FOR_PLAYERS_MESSAGE_HORIZONTAL_POSITION: f64 = 635.0;
        const WAITING_FOR_PLAYERS_MESSAGE_VERTICAL_POSITION: f64 = 500.0;
        const WAITING_FOR_PLAYERS_MESSAGE: &str = "Waiting for players...";
        Text::new_color(
            WHITE_COLOR,
            WAITING_FOR_PLAYERS_MESSAGE_FONT_SIZE,
        ).draw(
            WAITING_FOR_PLAYERS_MESSAGE,
            font,
            &context.draw_state,
            context.transform.trans(
                WAITING_FOR_PLAYERS_MESSAGE_HORIZONTAL_POSITION,
                WAITING_FOR_PLAYERS_MESSAGE_VERTICAL_POSITION,
            ),
            window
        ).unwrap();

        font.factory
            .encoder
            .flush(device);
    }
}
