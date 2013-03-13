extern mod glfw;
extern mod opengles;

mod input;
mod loader;
mod render;
mod scenes;
mod util;

// use input::Input;
// use loader::Obj;
// use util::println;

fn main() {
    // Run this task on the main thread. Unlike C or C++, a Rust program
    // automatically starts a new thread, so this line is _essential_ to ensure
    // that the OS is able to update the window and recieve events from the user.
    do task::spawn_sched(task::PlatformThread) {
        use core::unstable::finally::Finally;

        do (|| {
            glfw::set_error_callback(error_callback);

            if !glfw::init() { fail!(~"Failed to initialize GLFW\n"); }

            let (mode, monitor) = render::select_best_mode();

            let window =
                match glfw::Window::create(mode.width as uint, mode.height as uint, "Hello this is window", glfw::FullScreen(monitor)) {
                    Some(w) => w,
                    None    => fail!(~"Failed to open GLFW window")
                };

            window.set_key_callback(key_callback);
            window.make_context_current();

            // let obj = Obj::parse(~"./data/Banana.obj");

            render::init_gl();

            while !window.should_close() {
                glfw::poll_events();
            }

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
    io::println(fmt!("GLFW Error: %s", description));
}