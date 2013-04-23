use glfw;
// use scenefx = scenes::triangle;
// use scenefx = scenes::quad_obj;
use scenefx = scenes::quad_tex;
// use util::println;
use scene::*;
use screen;

#[main]
fn main() {
    do glfw::spawn ||
    {
        let (mode, _monitor) = screen::select_best_mode();

        // Choose a GL profile that is compatible with OS X 10.7+
        glfw::window_hint(glfw::CONTEXT_VERSION_MAJOR, 3);
        glfw::window_hint(glfw::CONTEXT_VERSION_MINOR, 2);
        glfw::window_hint(glfw::OPENGL_FORWARD_COMPAT, 1);
        glfw::window_hint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);
        glfw::window_hint(glfw::OPENGL_DEBUG_CONTEXT, glfw::TRUE);

        glfw::set_error_callback(on_error);
        // glDebugMessageCallbackARB()
        // glEnable(GL_DEBUG_OUTPUT);

        let windowMode = glfw::Windowed;
        // let windowMode = glfw::FullScreen(monitor);

        let window =
            match glfw::Window::create(mode.width as uint, mode.height as uint,
                "Hello this is window", windowMode) {
                Some(w) => w,
                None    => fail!(~"Failed to open GLFW window")
            };

        window.make_context_current();

        // HACK: force window to top
        window.iconify();
        window.restore();

        println(screen::gl_report());
        let ~scn = scenefx::init(mode.width, mode.height);
        let scene: @Scene = @scn;

        // window.set_key_callback(key_handler_gen(scene));

        do window.set_key_callback | window: &glfw::Window, key: libc::c_int, action: libc::c_int |
        {
            scene.on_key(key, action);
            application_controller(window, key, action);
        }

        while !window.should_close() {
            glfw::poll_events();
            scenefx::draw(scene);
            window.swap_buffers();
        }
    }
}

fn application_controller(window: &glfw::Window, key: libc::c_int, action: libc::c_int)
{
    if action == glfw::PRESS && key == glfw::KEY_ESCAPE
    {
        window.set_should_close(true);
    }
}

fn on_error(_error: libc::c_int, description: ~str)
{
    println(fmt!("GLFW Error: %s", description));
}