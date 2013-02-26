// use core::*;

extern mod sdl;
mod input;

use sdl;

use input;
use input::Input;

const SCREEN_WIDTH: u16  = 800;
const SCREEN_HEIGHT: u16 = 600;
const SCREEN_DEPTH: u16  = 32;

fn main() {
    #[main];

    do sdl::start {
        sdl::init([sdl::InitVideo]);
        sdl::wm::set_caption("Rust-SDL Lab", "rust-sdl");

        let video_report = sdl::video::set_video_mode(
            SCREEN_WIDTH as int,
            SCREEN_HEIGHT as int,
            SCREEN_DEPTH as int,
            [sdl::video::HWSurface],
            [sdl::video::DoubleBuf, sdl::video::Fullscreen]
        );

        let screen = match video_report
        {
            Ok(screen) => screen,
            Err(err) => fail!(fmt!("failed to set video mode: %s", err))
        };

        let rng = rand::Rng();
        // Note: You'll want to put this and the flip call inside the main loop
        // but we don't as to not startle epileptics
        let cubes_amount: u16 = 10;
        let cube_width        = (SCREEN_WIDTH / cubes_amount);
        let cube_height       = (SCREEN_HEIGHT / cubes_amount);

        for u16::range(0, cubes_amount) |i|
        {
            for u16::range(0, cubes_amount) |j|
            {
                screen.fill_rect(
                    Some(sdl::Rect
                    {
                        x: i * cube_width as i16,
                        y: j * cube_height as i16,
                        w: cube_width,
                        h: cube_height
                    }),
                    rng.gen::<sdl::video::Color>()
                );
            }
        }

        screen.flip();

        let mut input = Input::new();

        loop
        {
            match input.check_input()
            {
                input::Continue => {}
                input::Quit => break
            }
        }

        sdl::quit();
    }
}