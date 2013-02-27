// use core::*;
// use core::{libc, os, str};

extern mod sdl;
use sdl;
use sdlv = sdl::video;

mod gl;
use gl::*;

mod scenes;
use scenes::GridScene;

mod input;
use input::Input;

mod util;
use util::println;

struct Bounds
{
    w: u16,
    h: u16
}

// TODO: use bounds_available to find best resolution
const bounds_available: [Bounds * 3] = [
    Bounds { w: 1440, h: 900 },
    Bounds { w: 1152, h: 720 },
    Bounds { w: 1024, h: 640 }
];

// TODO: bit_depth_available to find best bit_depth
const bit_depth_available: [uint * 2] = [
    32,
    24
];

const CUBE_AMOUNT: u16 = 30;

fn main()
{
    #[main];

    do sdl::start
    {
        sdl::init([sdl::InitVideo]);
        sdl::wm::set_caption("Rust-SDL Lab", "rust-sdl");

        let bounds       = bounds_available[0];
        let bit_depth    = bit_depth_available[0];

        let video_report = sdl::video::set_video_mode(
            bounds.w as int,
            bounds.h as int,
            bit_depth as int,
            [sdlv::HWSurface],
            [sdlv::DoubleBuf, sdlv::Fullscreen, sdlv::OpenGL]
        );

        let surface = match video_report
        {
            Ok(surface) => surface,
            Err(err) => fail!(fmt!("failed to set video mode: %s", err))
        };

        unsafe
        {
            glMatrixMode(GL_PROJECTION);
            glOrtho(0.0, bounds.w as c_double, bounds.w as c_double, 0.0, 0.0, 1.0);
            glMatrixMode(GL_MODELVIEW);
            glLoadIdentity();
        }

        // let scene = GridScene::new(surface, CUBE_AMOUNT);
        let mut input = Input::new();

        loop
        {
            // scene.render(surface);
            surface.flip();

            match input.check_input()
            {
                input::Continue => {}
                input::Quit => break
            }
        }

        sdl::quit();
    }
}