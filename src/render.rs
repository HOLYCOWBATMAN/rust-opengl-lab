// use loader::Face;
// use loader::FaceTriplet;

// use core::io::Reader;

use core::io::ReaderUtil;
use gl = opengles::gl3;
// use gl = opengles::gl2;
use glfw;
use loader::Obj;
use util::println;
use core::sys::size_of;

struct RenderModel
{
    pgrm: gl::GLuint,
    frag_shdr: gl::GLuint,
    vert_shdr: gl::GLuint
}

#[inline(always)]
fn shader_path(file_name: &str) -> ~Path
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

pub fn init_gl(width: i32, height: i32) -> ~RenderModel
{
    // Create Vertex Array Object
    // let vao: GLuint = 0;
    let vao: gl::GLuint = gl::gen_vertex_arrays(1)[0];
    gl::bind_vertex_array(vao);

    // Create a Vertex Buffer Object and copy the vertex data to it
    let vbo: gl::GLuint = gl::gen_buffers(1)[0];

    let vertices: [gl::GLfloat * 15] = [
         0.0,  0.5,   1.0, 0.0, 0.0,
         0.5, -0.5,   0.0, 1.0, 0.0,
        -0.5, -0.5,   0.0, 0.0, 1.0
    ];

    gl::bind_buffer(gl::ARRAY_BUFFER, vbo);
    gl::buffer_data(gl::ARRAY_BUFFER, vertices, gl::STATIC_DRAW);

    let pgrm = gl::create_program();

    if pgrm == 0
    {
        fail!(~"Program done failed to create");
    }
    else
    {
        // TODO(BH): research a for comprehension style handling of Result/Options as in Scala
        let frag_shdr = attach_shader_from_file(pgrm, gl::FRAGMENT_SHADER, shader_path(~"unit.fs"));
        let vert_shdr = attach_shader_from_file(pgrm, gl::VERTEX_SHADER, shader_path(~"unit.vs"));

        gl::bind_frag_data_location(pgrm, 0, ~"outColor");

        match link_program(pgrm)
        {
            Ok(pgrm) => {
                gl::use_program(pgrm);

                // Specify the layout of the vertex data
                let posAttrib = gl::get_attrib_location(pgrm, ~"position");
                gl::enable_vertex_attrib_array(posAttrib);
                gl::vertex_attrib_pointer_f32(posAttrib, 2, false,
                                            5 * size_of::<gl::GLfloat>() as gl::GLsizei,
                                            0);

                let colAttrib = gl::get_attrib_location(pgrm, ~"color");
                gl::enable_vertex_attrib_array(colAttrib);
                gl::vertex_attrib_pointer_f32(colAttrib, 3, false,
                                            5 * size_of::<gl::GLfloat>() as gl::GLsizei,
                                            2 * size_of::<gl::GLfloat>() as gl::GLuint);

                gl::clear_color(0.1f32, 0.1f32, 0.1f32, 1f32);
                gl::viewport(0, 0, width, height);

                ~RenderModel
                {
                    pgrm: pgrm,
                    frag_shdr: frag_shdr,
                    vert_shdr: vert_shdr
                }
            },
            Err(msg) => fail!(msg)
        }
    }
}

pub fn draw(_model: &RenderModel)
{
    gl::clear(gl::COLOR_BUFFER_BIT);
    gl::draw_arrays(gl::TRIANGLES, 0, 3);
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
fn link_program(pgrm: gl::GLuint) -> Result<gl::GLuint, ~str>
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

fn attach_shader_from_file(pgrm: gl::GLuint, shader_type: gl::GLenum, file_path: &Path) -> gl::GLuint
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

pub fn render_obj(obj: &Obj)
{
    // glClear( GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT );

    // glNewList(list, GL_COMPILE);

    for obj.faces.each |&face| {
        println(fmt!("face"));
        for face.triplets.each |&triplet| {
            println(fmt!("\t%?", triplet));
        }
    }

    // glEndList();
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

// fn draw_face(obj: &Obj, face: &Face)
// {
//     let tl = len(face.triplets);

//     if (tl == 3)
//     { // triangle
//         draw_tri(obj, face);
//     }
//     else if (tl == 4)
//     { // quad
//         draw_quad(obj, face);
//     }
//     else
//     {
//         fail!(fmt!());
//     }
// }

// fn draw_tri(obj: &Obj, face: &Face)
// {
//     let has_normals = len(face.normal) == 3;

//     if (has_normals)
//     { // with normals
//         glBegin(GL_TRIANGLES);
//         glNormal3f(normals[face.normal[0]].v[0], normals[face.normal[0]].v[1], normals[face.normal[0]].v[2]);
//         glVertex3f(vertices[face.vertex[0]].v[0], vertices[face.vertex[0]].v[1], vertices[face.vertex[0]].v[2]);
//         glNormal3f(normals[face.normal[1]].v[0], normals[face.normal[1]].v[1], normals[face.normal[1]].v[2]);
//         glVertex3f(vertices[face.vertex[1]].v[0], vertices[face.vertex[1]].v[1], vertices[face.vertex[1]].v[2]);
//         glNormal3f(normals[face.normal[2]].v[0], normals[face.normal[2]].v[1], normals[face.normal[2]].v[2]);
//         glVertex3f(vertices[face.vertex[2]].v[0], vertices[face.vertex[2]].v[1], vertices[face.vertex[2]].v[2]);
//         glEnd();
//     }
//     else
//     { // without normals -- evaluate normal on triangle
//         vertex v = (vertices[face.vertex[1]] - vertices[face.vertex[0]]).cross(vertices[face.vertex[2]] - vertices[face.vertex[0]]);
//         v.normalize();
//         glBegin(GL_TRIANGLES);
//         glNormal3f(v.v[0], v.v[1], v.v[2]);
//         glVertex3f(vertices[face.vertex[0]].v[0], vertices[face.vertex[0]].v[1], vertices[face.vertex[0]].v[2]);
//         glVertex3f(vertices[face.vertex[1]].v[0], vertices[face.vertex[1]].v[1], vertices[face.vertex[1]].v[2]);
//         glVertex3f(vertices[face.vertex[2]].v[0], vertices[face.vertex[2]].v[1], vertices[face.vertex[2]].v[2]);
//         glEnd();
//     }
// }

// fn draw_quad(obj: &Obj, face: &Face)
// {
//     if (face.normal.size() == 4)
//     { // with normals
//         glBegin(GL_QUADS);
//         glNormal3f(normals[face.normal[0]].v[0], normals[face.normal[0]].v[1], normals[face.normal[0]].v[2]);
//         glVertex3f(vertices[face.vertex[0]].v[0], vertices[face.vertex[0]].v[1], vertices[face.vertex[0]].v[2]);
//         glNormal3f(normals[face.normal[1]].v[0], normals[face.normal[1]].v[1], normals[face.normal[1]].v[2]);
//         glVertex3f(vertices[face.vertex[1]].v[0], vertices[face.vertex[1]].v[1], vertices[face.vertex[1]].v[2]);
//         glNormal3f(normals[face.normal[2]].v[0], normals[face.normal[2]].v[1], normals[face.normal[2]].v[2]);
//         glVertex3f(vertices[face.vertex[2]].v[0], vertices[face.vertex[2]].v[1], vertices[face.vertex[2]].v[2]);
//         glNormal3f(normals[face.normal[3]].v[0], normals[face.normal[3]].v[1], normals[face.normal[3]].v[2]);
//         glVertex3f(vertices[face.vertex[3]].v[0], vertices[face.vertex[3]].v[1], vertices[face.vertex[3]].v[2]);
//         glEnd();
//     }
//     else
//     { // without normals -- evaluate normal on quad
//         vertex v = (vertices[face.vertex[1]] - vertices[face.vertex[0]]).cross(vertices[face.vertex[2]] - vertices[face.vertex[0]]);
//         v.normalize();
//         glBegin(GL_QUADS);
//         glNormal3f(v.v[0], v.v[1], v.v[2]);
//         glVertex3f(vertices[face.vertex[0]].v[0], vertices[face.vertex[0]].v[1], vertices[face.vertex[0]].v[2]);
//         glVertex3f(vertices[face.vertex[1]].v[0], vertices[face.vertex[1]].v[1], vertices[face.vertex[1]].v[2]);
//         glVertex3f(vertices[face.vertex[2]].v[0], vertices[face.vertex[2]].v[1], vertices[face.vertex[2]].v[2]);
//         glVertex3f(vertices[face.vertex[3]].v[0], vertices[face.vertex[3]].v[1], vertices[face.vertex[3]].v[2]);
//         glEnd();
//     }
// }