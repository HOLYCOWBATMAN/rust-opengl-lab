extern mod glfw;
extern mod lmath;
extern mod numeric;
extern mod opengles;
extern mod stb_image;
extern mod std;

// use scenefx = scenes::triangle;
use scenefx = scenes::quad_tex;
// use scenefx = scenes::quad_obj;
// use util::println;

// mod input;
// #[path = "io/mod.rs"]
// mod io;
pub mod math;
#[path = "scenes/mod.rs"]
mod scenes;
mod screen;
mod util;
pub mod config;
pub mod scene;

#[main]
fn main() {
    do glfw::spawn ||
    {
        glfw::set_error_callback(error_callback);

        let (mode, monitor) = screen::select_best_mode();

        // Choose a GL profile that is compatible with OS X 10.7+
        glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
        glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 2);
        glfw::window_hint(glfw::OPENGL_FORWARD_COMPAT, 1);
        glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);

        let window =
            match glfw::Window::create(mode.width as uint, mode.height as uint, "Hello this is window", glfw::FullScreen(monitor)) {
                Some(w) => w,
                None    => fail!(~"Failed to open GLFW window")
            };

        window.set_key_callback(key_callback);
        window.make_context_current();

        println(screen::gl_report());
        let scn = scenefx::init(mode.width, mode.height);

        while !window.should_close() {
            glfw::poll_events();
            scenefx::draw(scn);
            window.swap_buffers();
        }

        scene::destroy(scn);
    }
}

fn key_callback(window: &glfw::Window, key: libc::c_int, action: libc::c_int) {
    if action == glfw::PRESS && key == glfw::KEY_ESCAPE {
        window.set_should_close(true);
    }
}

fn error_callback(_error: libc::c_int, description: ~str) {
    println(fmt!("GLFW Error: %s", description));
}