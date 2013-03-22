// use gl = opengles::gl2;
use core::io::ReaderUtil;
use gl = opengles::gl3;
use glfw;

pub struct Scene
{
    pgrm: gl::GLuint,
    frag_shdr: gl::GLuint,
    vert_shdr: gl::GLuint
}

#[inline(always)]
pub fn shader_path(file_name: &str) -> ~Path
{
    ~PosixPath(str::append(~"data/shaders/glsl/", file_name))
}

pub fn gl_report() -> ~str
{
    let glfw_version = glfw::get_version();
    let gl_version   = gl::get_string(gl::VERSION);
    let sl_ver       = gl::get_string(gl::SHADING_LANGUAGE_VERSION);

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
    let modes     = monitor.get_video_modes();
    let mode_best = *vec::last(modes);

    (mode_best, monitor)
}

pub fn load_shader(shader_type: gl::GLenum, file_path: &Path) -> Result<gl::GLuint, ~str>
{
    do read_file(file_path).chain |file_bytes|
    {
        let shader = gl::create_shader(shader_type);

        if (shader == 0)
        {
            Err(~"Shader creation Error")
        }
        else
        {
            gl::shader_source(shader, file_bytes);
            gl::compile_shader(shader);

            let status = gl::get_shader_iv(shader, gl::COMPILE_STATUS);

            match status
            {
                0 => {
                    let log_entry = gl::get_shader_info_log(shader);
                    gl::delete_shader(shader);
                    Err(log_entry)
                },
                _ => {
                    Ok(shader)
                }
            }
        }
    }
}

// fn link_program(pgrm: gl::GLuint, bindings: ~[~str]) -> Result<gl::GLuint, ~str>
pub fn link_program(pgrm: gl::GLuint) -> Result<gl::GLuint, ~str>
{
    // for bindings.eachi |i, &var_name|
    // {
        //gl::bind_attrib_location(pgrm, i as u32, var_name);
    // }

    gl::link_program(pgrm);

    match gl::get_program_iv(pgrm, gl::LINK_STATUS)
    {
        0 => {
            let log_entry = gl::get_program_info_log(pgrm);
            gl::delete_program(pgrm);
            Err(log_entry)
        }
        _ => Ok(pgrm)
    }
}

pub fn attach_shader_from_file(pgrm: gl::GLuint, shader_type: gl::GLenum, file_path: &Path) -> gl::GLuint
{
    unwrap(
        do load_shader(shader_type, file_path).map |&shdr|
        {
            gl::attach_shader(pgrm, shdr);
            shdr
        },
        |msg| fmt!("Shader %s in file: %s", msg, file_path.to_str())
    )
}

pub fn read_file(file_path: &Path) -> Result<~[~[u8]], ~str>
{
    do io::file_reader(file_path).map |file|
    {
        let mut out = ~[];

        while !file.eof()
        {
            out += file.read_bytes(2048u);
        }
        out += [0u8];

        ~[out]
    }
}

/// Unwraps a result, assuming it is an `ok(T)`
#[inline(always)]
pub pure fn unwrap<T>(res: Result<T, ~str>, append: fn (&str) -> ~str) -> T {
    match res {
      Ok(t) => t,
      Err(msg_callee) =>
        fail!(append(msg_callee))
    }
}