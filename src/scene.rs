use core::io::ReaderUtil;
use core::str::from_bytes;
use core::vec::from_elem;
use core::vec::raw::to_ptr;

use camera::*;
use glcore::*;
// use glfw;

fn destroy_T<T>(_x: T) {
    // Just let the object drop.
}

pub struct Scene
{
    programs: ~[ShaderProgram],
    models: ~[Model],
    camera: Camera
}

pub trait KeyInputHandler
{
    fn on_key(&self, key: libc::c_int, action: libc::c_int);
}

impl KeyInputHandler for Scene
{
    fn on_key(&self, _key: libc::c_int, _action: libc::c_int)
    {

    }
}

impl Drop for Scene
{
    pub fn finalize(&self)
    {
        for self.models.each() |&model|
        {
            let t_len = model.textures.len();
            if t_len > 0
            {
                glDeleteTextures(t_len as GLint, &model.textures[0]);
            }

            let b_len = model.buffers.len();
            if b_len > 0
            {
                glDeleteBuffers(b_len as GLint, &model.buffers[0]);
            }

            let v_len = model.vertex_arrays.len();
            if v_len > 0
            {
                glDeleteVertexArrays(v_len as GLint, &model.vertex_arrays[0]);
            }
        }

        for self.programs.each() |&program|
        {
            for program.shaders.each() |&shader|
            {
                glDetachShader(program.id, shader);
                glDeleteShader(shader);
            }

            glDeleteProgram(program.id);
        }
    }
}

pub struct Model
{
    buffers: ~[GLuint],
    vertex_arrays: ~[GLuint],
    element_count: uint,
    textures: ~[GLuint]
}

pub struct ShaderProgram
{
    id: GLuint,
    shaders: ~[GLuint],
    uniforms: ~[GLint]
}

pub fn load_shader(shader_type: GLenum, file_path: &Path) -> Result<GLuint, ~str>
{
    do read_file(file_path).chain |file_lines|
    {
        let shader = glCreateShader(shader_type);

        if (shader == 0)
        {
            Err(~"Shader creation Error")
        }
        else
        {
            unsafe {
                let pointers = file_lines.map(|file_lines| to_ptr(*file_lines));
                let lengths  = file_lines.map(|file_lines| file_lines.len() as GLint);
                glShaderSource(shader, pointers.len() as GLsizei,
                                   to_ptr(pointers) as **GLchar, to_ptr(lengths));
                destroy_T(lengths);
                destroy_T(pointers);
            }

            glCompileShader(shader);

            let status: GLint = 0;
            glGetShaderiv(shader, GL_COMPILE_STATUS, &status);

            match status
            {
                0 => {
                    let log_entry = get_shader_info_log(shader);
                    glDeleteShader(shader);
                    Err(log_entry)
                },
                _ => {
                    Ok(shader)
                }
            }
        }
    }
}

pub fn get_shader_info_log(shader: GLuint) -> ~str {
    unsafe {
        let result = from_elem(1024u, 0u8);
        let result_len: GLsizei = 0 as GLsizei;
        glGetShaderInfoLog(shader,
                               1024 as GLsizei,
                               &result_len,
                               to_ptr(result) as *GLchar);
        return from_bytes(result);
    }
}

pub fn get_program_info_log(program: GLuint) -> ~str {
    unsafe {
        let result = from_elem(1024u, 0u8);
        let result_len: GLsizei = 0 as GLsizei;
        glGetProgramInfoLog(program,
                               1024 as GLsizei,
                               &result_len,
                               to_ptr(result) as *GLchar);
        return from_bytes(result);
    }
}

// fn link_program(pgrm: gl::GLuint, bindings: ~[~str]) -> Result<gl::GLuint, ~str>
pub fn link_program(pgrm: GLuint) -> Result<GLuint, ~str>
{
    // for bindings.eachi |i, &var_name|
    // {
        //gl::bind_attrib_location(pgrm, i as u32, var_name);
    // }

    glLinkProgram(pgrm);
    let link_status: GLint = 0;
    glGetProgramiv(pgrm, GL_LINK_STATUS, &link_status);

    match link_status
    {
        0 => {
            let log_entry = get_program_info_log(pgrm);
            glDeleteProgram(pgrm);
            Err(log_entry)
        }
        _ => Ok(pgrm)
    }
}

pub fn attach_shader_from_file(pgrm: GLuint, shader_type: GLenum, file_path: &Path) -> GLuint
{
    unwrap(
        do load_shader(shader_type, file_path).map |&shdr|
        {
            glAttachShader(pgrm, shdr);
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
pub fn unwrap<T>(res: Result<T, ~str>, append: &fn (&str) -> ~str) -> T {
    match res {
      Ok(t) => t,
      Err(msg_callee) =>
        fail!(append(msg_callee))
    }
}