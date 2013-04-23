use glcore::*;
use glfw;

pub fn gl_report() -> ~str
{
    let glfw_version = glfw::get_version();
    let gl_version   = unsafe { str::raw::from_buf(glGetString(GL_VERSION)) };
    let sl_ver       = unsafe { str::raw::from_buf(glGetString(GL_SHADING_LANGUAGE_VERSION)) };

    fmt!(
    "
    GLFW version:   %i.%i.%i
    OpenGL version: %s
    GLSL version:   %s
    ",
        glfw_version.major, glfw_version.minor, glfw_version.rev,
        gl_version,
        sl_ver
    )
}

pub fn select_best_mode() -> (glfw::VidMode, glfw::Monitor)
{
    // glfw::get_monitors().map(|monitor| {
    //     io::println(fmt!("%s:", monitor.get_name()));

    //     do monitor.get_video_modes().map |mode| {
    //         io::println(fmt!("  %s", mode.to_str()));
    //     }
    // });

    let monitor   = glfw::get_primary_monitor();
    // let modes     = monitor.get_video_modes();
    // let mode_best = *vec::last(modes);

    let mode_best = glfw::VidMode {
        width      : 1024,
        height     : 768,
        redBits    : 8,
        blueBits   : 8,
        greenBits  : 8,
    };

    (mode_best, monitor)
}