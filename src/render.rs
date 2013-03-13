// use loader::Face;
// use loader::FaceTriplet;
use gl = opengles::gl2;
use glfw;
use loader::Obj;
use util::println;
use core::io::ReaderUtil;

pub fn read_file(file_path: ~str) -> Result<~[~[u8]], ~str>
{
    let pth = &path::Path(file_path);
    do io::file_reader(pth).map |file| {
        let mut out = ~[];

        while !file.eof()
        {
            out.push(str::to_bytes(file.read_line()));
        }

        out
    }
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

pub fn load_shader(shader_type: gl::GLenum, file_path: ~str) -> Result<gl::GLuint, ~str>
{
    do read_file(file_path).chain |file_lines|
    {
        let shader = gl::create_shader(shader_type);
        gl::shader_source(shader, file_lines);
        gl::compile_shader(shader);
        let is_compiled = gl::get_shader_iv(shader, gl::COMPILE_STATUS);

        if is_compiled > 0
        {
            Ok(shader)
        }
        else
        {
            Err(gl::get_shader_info_log(shader))
        }
    }
}

pub fn init_gl()
{
    let pgrm = gl::create_program();

    match load_shader(gl::VERTEX_SHADER, ~"data/unit.vs")
    {
        Ok(vert_sh) => gl::attach_shader(pgrm, vert_sh),
        Err(msg) => fail!(msg)
    }

    match load_shader(gl::FRAGMENT_SHADER, ~"data/unit.fs")
    {
        Ok(frag_sh) => gl::attach_shader(pgrm, frag_sh),
        Err(msg) => fail!(msg)
    }

    // gl::bind_attrib_location();
    gl::link_program(pgrm);
    let link_status = gl::get_program_iv(pgrm, gl::LINK_STATUS);

    if link_status > 0
    {
        gl::clear_color(0f32, 0f32, 0f32, 1f32);
    }
    else
    {
        // fail!(gl::get_program_info_log(pgrm));
    }
}

fn bool_from_int(n: gl::GLint) -> bool
{
    n > 0
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