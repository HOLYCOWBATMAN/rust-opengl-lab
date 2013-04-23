use core::cast::transmute;
use core::f32;
use core::sys::size_of;
use core::vec::from_elem;
use core::vec::raw::to_ptr;

use camera::*;

use glcore::*;
use glfw;
use lmath::projection::*;
use lmath::mat::*;
use lmath::vec::*;

use camera::*;
use config;
use imageio = stb_image::image;
use numeric::float::Float::*;
use scene;
// use util::println;

static UNIFORM_TEX_CONST: uint  = 0;
static UNIFORM_MAT4_MODEL: uint = 1;
static UNIFORM_MAT4_VIEW: uint  = 2;
static UNIFORM_MAT4_PROJ: uint  = 3;

pub fn init(width: i32, height: i32) -> ~scene::Scene
{
    // glDisable(GL_DEPTH_TEST);

    // Create Vertex Array Object
    let mut vao: GLuint = 0;
    glGenVertexArrays(1, &vao);
    glBindVertexArray(vao);

    // Create a Vertex Buffer Object and copy the vertex data to it
    let vbo: GLuint = 0;
    glGenBuffers(1, &vbo);

    let vertices: [GLfloat, ..32] = [
    //   Position         Color            Texcoords
        -0.5,  0.5, 0.0,  1.0, 0.0, 0.0,   0.0, 0.0, // Top-left
         0.5,  0.5, 0.0,  0.0, 1.0, 0.0,   1.0, 0.0, // Top-right
         0.5, -0.5, 0.0,  0.0, 0.0, 1.0,   1.0, 1.0, // Bottom-right
        -0.5, -0.5, 0.0,  1.0, 1.0, 1.0,   0.0, 1.0  // Bottom-left
    ];

    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    unsafe {
        glBufferData(GL_ARRAY_BUFFER,
                     (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
                     cast::transmute(&vertices),
                     GL_STATIC_DRAW);
    }

    // Create an element array
    let ebo: GLuint = 0;
    glGenBuffers(1, &ebo);

    let elements: [GLuint, ..6] = [
        0, 1, 2,
        2, 3, 0
    ];

    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
    unsafe {
        glBufferData(GL_ELEMENT_ARRAY_BUFFER,
                     (elements.len() * size_of::<GLfloat>()) as GLsizeiptr,
                     cast::transmute(&elements),
                     GL_STATIC_DRAW);
    }

    let shader_program: GLuint = glCreateProgram();

    if shader_program == 0
    {
        fail!(~"Program done failed to create");
    }
    else
    {
        let frag_shdr = scene::attach_shader_from_file(shader_program, GL_FRAGMENT_SHADER, config::shader_path(~"tex.fs"));
        let vert_shdr = scene::attach_shader_from_file(shader_program, GL_VERTEX_SHADER, config::shader_path(~"tex.vs"));

        glBindFragDataLocation(shader_program, 0, str::as_c_str("outColor", |s|s));

        match scene::link_program(shader_program)
        {
            Ok(shader_program) => {
                glUseProgram(shader_program);

                let stride       = 8 * size_of::<GLfloat>() as GLsizei;
                let color_offset = 2 * size_of::<GLfloat>() as uint;
                let tex_offset   = 6 * size_of::<GLfloat>() as uint;

                // Specify the layout of the vertex data
                let pos_attrib = glGetAttribLocation(shader_program, str::as_c_str("position", |s|s)) as GLuint;
                glEnableVertexAttribArray(pos_attrib);
                glVertexAttribPointer(pos_attrib, 3, GL_FLOAT, GL_FALSE, stride, ptr::null());

                let col_attrib = glGetAttribLocation(shader_program, str::as_c_str("color", |s|s)) as GLuint;
                glEnableVertexAttribArray(col_attrib);
                unsafe {
                    glVertexAttribPointer(col_attrib, 3, GL_FLOAT, GL_FALSE, stride,
                                          cast::transmute(color_offset));
                }

                let tex_attrib = glGetAttribLocation(shader_program, str::as_c_str("texcoord", |s|s)) as GLuint;
                glEnableVertexAttribArray(tex_attrib);
                unsafe {
                    glVertexAttribPointer(tex_attrib, 2, GL_FLOAT, GL_FALSE, stride,
                                          cast::transmute(tex_offset));
                }

                let proj_loc      = glGetUniformLocation(shader_program, str::as_c_str("proj", |s|s));
                let view_loc      = glGetUniformLocation(shader_program, str::as_c_str("view", |s|s));
                let model_loc     = glGetUniformLocation(shader_program, str::as_c_str("model", |s|s));
                let tex_alpha_loc = glGetUniformLocation(shader_program, str::as_c_str("texAlpha", |s|s));

                let tex_names = load_textures(shader_program, ~[
                    (~"data/models/quad/huis1.png", ~"texHuis"),
                    (~"data/models/banana/Banana.jpg", ~"texBanana")]);

                glViewport(0, 0, width, height);

                let program = scene::ShaderProgram {
                    id: shader_program,
                    shaders: ~[frag_shdr, vert_shdr],
                    uniforms: ~[tex_alpha_loc, model_loc, view_loc, proj_loc]
                };

                let _0 = 0f32;
                let _1 = 1f32;
                let eye    = vec3::new( 0f32, 1.2f32, -1.2f32 );
                let target = vec3::new( _0, _0, _0 );
                let up     = vec3::new( _0, _1, _0 );

                let view = look_at(&eye, &target, &up);
                let ratio: f32 = width as f32 / (height as f32);
                let proj  = perspective(90.0f32, ratio, 1.0f32, 100.0f32);

                println(fmt!("view: %?", view));

                glUniformMatrix4fv(proj_loc, 1, GL_FALSE as u8, proj.to_ptr());
                glUniformMatrix4fv(view_loc, 1, GL_FALSE as u8, view.to_ptr());

                let model = scene::Model {
                    buffers: ~[vbo],
                    vertex_arrays: ~[vao],
                    element_count: elements.len(),
                    textures: tex_names
                };

                ~scene::Scene
                {
                    programs: ~[program],
                    models: ~[model],
                    camera: Camera::look_at(
                        &vec3::new(0f32, 0f32, -1f32),
                        &vec3::identity(),
                        &vec3::unit_y()
                    )
                }
            },
            Err(msg) => fail!(msg)
        }
    }
}

fn look_at(eye: &vec3, target: &vec3, up: &vec3) -> mat4 {

    let vdir     = target.sub_v(eye).normalize();
    let vup      = up.sub_v(&vdir.mul_t(up.dot(&vdir))).normalize();
    let vside    = vdir.cross(&vup);

    let rot      = mat3::from_cols(vside, vup, vdir.neg());
    let eyeInv   = rot.mul_v(eye).neg();

    let mut view = rot.to_mat4();
    let mut eye  = view.col_mut(3);
    eye.x        = eyeInv.x;
    eye.y        = eyeInv.y;
    eye.z        = eyeInv.z;

    view
}

fn load_textures(pgrm: GLuint, path_bind_tpl: ~[(~str, ~str)]) -> ~[GLuint]
{
    let tex_amount = path_bind_tpl.len();
    let tex_names  = from_elem(tex_amount as uint, 0 as GLuint);
    unsafe { glGenTextures(tex_amount as GLsizei, to_ptr(tex_names)); }

    for path_bind_tpl.eachi |idx, &tpl| {
        match tpl {
            (image_path, binding) => load_texture(pgrm, idx, tex_names[idx], image_path, binding)
        }
    }

    tex_names
}

fn load_texture(pgrm: GLuint, tex_index: uint, tex_name: GLuint, image_path: ~str, bind_name: ~str)
{
    match imageio::load_with_depth(image_path, 3, false)
    {
        imageio::Error => fail!(~"error loading image"),
        imageio::ImageF32(_) => fail!(~"error: F32 image format is not supported"),
        imageio::ImageU8(img) => {

            let tex_trg  = GL_TEXTURE_2D;
            let tex_slot = GL_TEXTURE0 + tex_index as u32;

            glActiveTexture(tex_slot);
            glBindTexture(tex_trg, tex_name);

            let u_loc = glGetUniformLocation(pgrm, str::as_c_str(bind_name, |s|s));
            glUniform1i(u_loc, tex_index as GLint);

            unsafe {
                glTexImage2D(
                    tex_trg,
                    0,
                    GL_RGB as GLint,
                    img.width as GLsizei,
                    img.height as GLsizei,
                    0,
                    GL_RGB,
                    GL_UNSIGNED_BYTE,
                    transmute(to_ptr(img.data))
                );
            }

            glTexParameteri(tex_trg, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as GLint);
            glTexParameteri(tex_trg, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as GLint);
            glTexParameteri(tex_trg, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
            glTexParameteri(tex_trg, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);
        }
    }
}

pub fn draw(scene: &scene::Scene)
{
    let pgrm          = &scene.programs[0];
    let time          = glfw::get_time() as f32;
    let tex_alpha_loc = pgrm.uniforms[UNIFORM_TEX_CONST];
    let u             = ((1f32 + f32::cos(time)) * 0.5f32);
    glUniform1f(tex_alpha_loc as GLint, u);

    let mat4_model_loc = pgrm.uniforms[UNIFORM_MAT4_MODEL];
    // let mat4_view_loc  = pgrm.uniforms[UNIFORM_MAT4_VIEW];
    // let mat4_proj_loc  = pgrm.uniforms[UNIFORM_MAT4_PROJ];

    let rad   = time * 0.5f32 * pi();

    // let model = mat4::identity();

    let mut model = mat3::from_angle_y(rad).to_mat4();
    {
        let mut trans = model.col_mut(3);
        trans.x = 0f32;
        trans.y = 0f32;
        trans.z = 0f32;
    }

    glUniformMatrix4fv(mat4_model_loc, 1, GL_FALSE as u8, model.to_ptr());

    let model = &scene.models[0];
    glClearColor(0.1, 0.1, 0.1, 1.0);
    glClear(GL_COLOR_BUFFER_BIT);
    glDrawElements(GL_TRIANGLES, model.element_count as GLsizei, GL_UNSIGNED_INT, ptr::null());
}