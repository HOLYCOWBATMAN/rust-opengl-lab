extern mod glfw;
extern mod opengles;
extern mod std;
extern mod stb_image;

pub mod config;
mod input;
mod loader;
pub mod scene;
mod screen;
mod util;
#[path = "scenes/mod.rs"]
mod scenes;

// use scenefx = scenes::triangle;
use scenefx = scenes::triangle_tex;
use util::println;

fn main() {
    // Run this task on the main thread. Unlike C or C++, a Rust program
    // automatically starts a new thread, so this line is _essential_ to ensure
    // that the OS is able to update the window and recieve events from the user.
    do task::spawn_sched(task::PlatformThread) {
        use core::unstable::finally::Finally;

        do (|| {
            glfw::set_error_callback(error_callback);

            if !glfw::init()
            {
                glfw::terminate();
                fail!(~"Failed to initialize GLFW\n");
            }

            let (mode, monitor) = screen::select_best_mode();

            // Choose a GL profile that is compatible with OS X 10.7+
            glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
            glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 2);
            glfw::window_hint(glfw::OPENGL_FORWARD_COMPAT, 1);
            glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);
            // glfw::window_hint(glfw::CLIENT_API, glfw::OPENGL_ES_API);

            let window =
                match glfw::Window::create(mode.width as uint, mode.height as uint, "Hello this is window", glfw::FullScreen(monitor)) {
                    Some(w) => w,
                    None    => fail!(~"Failed to open GLFW window")
                };

            window.set_key_callback(key_callback);
            window.make_context_current();

            println(screen::gl_report());
            // scene::init();
            let scn = scenefx::init(mode.width, mode.height);

            while !window.should_close() {
                glfw::poll_events();
                scenefx::draw(scn);
                window.swap_buffers();
            }

            scene::destroy(scn);

        }).finally {

            glfw::terminate();    // terminate glfw on completion
        }
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