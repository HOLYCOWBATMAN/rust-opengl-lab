use config;
use core::sys::size_of;
use glcore::*;
use glfw;
use imageio = stb_image::image;
use lmath::mat::*;
use lmath::vec::*;
use math = core::f32;
use numeric::float::Float::*;
use scene;
// use util::println;

static UNIFORM_TEX_CONST: uint  = 0;
static UNIFORM_MAT4_TRANS: uint = 1;

pub fn init(width: i32, height: i32) -> ~scene::Scene
{
    // Create Vertex Array Object
    let vao: gl::GLuint = gl::gen_vertex_arrays(1)[0];
    gl::bind_vertex_array(vao);

    // Create a Vertex Buffer Object and copy the vertex data to it
    let vbo: gl::GLuint = gl::gen_buffers(1)[0];

    let vertices: [gl::GLfloat, ..28] = [
    //   Position     Color            Texcoords
        -0.5,  0.5,   1.0, 0.0, 0.0,   0.0, 0.0, // Top-left
         0.5,  0.5,   0.0, 1.0, 0.0,   1.0, 0.0, // Top-right
         0.5, -0.5,   0.0, 0.0, 1.0,   1.0, 1.0, // Bottom-right
        -0.5, -0.5,   1.0, 1.0, 1.0,   0.0, 1.0  // Bottom-left
    ];

    gl::bind_buffer(gl::ARRAY_BUFFER, vbo);
    gl::buffer_data(gl::ARRAY_BUFFER, vertices, gl::STATIC_DRAW);

    // Create an element array
    let ebo: gl::GLuint = gl::gen_buffers(1)[0];

    let elements: [gl::GLuint, ..6] = [
        0, 1, 2,
        2, 3, 0
    ];

    gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::buffer_data(gl::ELEMENT_ARRAY_BUFFER, elements, gl::STATIC_DRAW);

    let pgrm = gl::create_program();

    if pgrm == 0
    {
        fail!(~"Program done failed to create");
    }
    else
    {
        let frag_shdr = scene::attach_shader_from_file(pgrm, gl::FRAGMENT_SHADER, config::shader_path(~"tex.fs"));
        let vert_shdr = scene::attach_shader_from_file(pgrm, gl::VERTEX_SHADER, config::shader_path(~"tex.vs"));

        gl::bind_frag_data_location(pgrm, 0, ~"outColor");

        match scene::link_program(pgrm)
        {
            Ok(pgrm) => {
                gl::use_program(pgrm);

                let stride       = 7 * size_of::<gl::GLfloat>() as gl::GLsizei;
                let color_offset = 2 * size_of::<gl::GLfloat>() as gl::GLuint;
                let tex_offset   = 5 * size_of::<gl::GLfloat>() as gl::GLuint;

                // Specify the layout of the vertex data
                let posAttrib = gl::get_attrib_location(pgrm, ~"position");
                gl::enable_vertex_attrib_array(posAttrib);
                gl::vertex_attrib_pointer_f32(posAttrib, 2, false, stride, 0);

                let colAttrib = gl::get_attrib_location(pgrm, ~"color");
                gl::enable_vertex_attrib_array(colAttrib);
                gl::vertex_attrib_pointer_f32(colAttrib, 3, false, stride, color_offset);

                let texAttrib = gl::get_attrib_location(pgrm, ~"texcoord");
                gl::enable_vertex_attrib_array(texAttrib);
                gl::vertex_attrib_pointer_f32(texAttrib, 2, false, stride, tex_offset);

                // let _proj = glm::perspective( 45.0f32, 800.0f32 / 600.0f32, 1.0f32, 10.0f32 );

                let projLoc      = gl::get_uniform_location(pgrm, ~"proj");
                let mat4TransLoc = gl::get_uniform_location(pgrm, ~"trans");
                let texAlphaLoc  = gl::get_uniform_location(pgrm, ~"texAlpha");

                // gl::ll::glUniformMatrix4fv(projLoc, 1, gl::FALSE as u8, proj.to_mat4().to_ptr());

                let tex_names = load_textures(pgrm, ~[
                    (~"data/models/quad/huis1.png", ~"texHuis"),
                    (~"data/models/banana/Banana.jpg", ~"texBanana")]);

                gl::clear_color(0.1f32, 0.1f32, 0.1f32, 1f32);
                gl::viewport(0, 0, width, height);

                let program = scene::ShaderProgram {
                    id: pgrm,
                    shaders: ~[frag_shdr, vert_shdr],
                    uniforms: ~[texAlphaLoc, mat4TransLoc, projLoc]
                };

                let model = scene::Model {
                    buffers: ~[vbo],
                    vertex_arrays: ~[vao],
                    element_count: elements.len(),
                    textures: tex_names
                };

                ~scene::Scene
                {
                    programs: ~[program],
                    models: ~[model]
                }
            },
            Err(msg) => fail!(msg)
        }
    }
}

fn position_scene()
{
    // let trans = mat3::from_angle_z(frac_pi_2());
    // let vA = Vec3{ x:1.0f, y:2.0f, z:0.0f };
    // let vB = Vec3{ x:0.0f, y:2.0f, z:1.0f };

    // let vC = vA.add_v(&vB);
    // trans.
    // println(fmt!("vC: %?", vC));
}

fn load_textures(pgrm: gl::GLuint, path_bind_tpl: ~[(~str, ~str)]) -> ~[gl::GLuint]
{
    let tex_amount = path_bind_tpl.len();
    let tex_names  = gl::gen_textures(tex_amount as gl::GLsizei);

    for path_bind_tpl.eachi |idx, &tpl| {
        match tpl {
            (image_path, binding) => load_texture(pgrm, idx, tex_names[idx], image_path, binding)
        }
    }

    tex_names
}

fn load_texture(pgrm: gl::GLuint, tex_index: uint, tex_name: gl::GLuint, image_path: ~str, bind_name: ~str)
{
    match imageio::load_with_depth(image_path, 3, false)
    {
        imageio::Error => fail!(~"error loading image"),
        imageio::ImageF32(_) => fail!(~"error: F32 image format is not supported"),
        imageio::ImageU8(img) => {

            let d: &[u8] = img.data;
            let tex_trg  = gl::TEXTURE_2D;
            let tex_slot = gl::TEXTURE_SLOT[tex_index];

            gl::active_texture(tex_slot);
            gl::bind_texture(tex_trg, tex_name);

            let u_loc = gl::get_uniform_location(pgrm, bind_name);
            gl::uniform_1i(u_loc, tex_index as gl::GLint);

            gl::tex_image_2d(
                tex_trg,
                0,
                gl::RGB as gl::GLint,
                img.width as gl::GLsizei,
                img.height as gl::GLsizei,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                Some(d)
            );

            gl::tex_parameter_i(tex_trg, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as gl::GLint);
            gl::tex_parameter_i(tex_trg, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as gl::GLint);
            gl::tex_parameter_i(tex_trg, gl::TEXTURE_MIN_FILTER, gl::LINEAR as gl::GLint);
            gl::tex_parameter_i(tex_trg, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::GLint);
        }
    }
}

pub fn draw(scene: &scene::Scene)
{
    let pgrm        = &scene.programs[0];
    let time        = glfw::get_time() as f32;
    let texAlphaLoc = pgrm.uniforms[UNIFORM_TEX_CONST];
    let u           = ((1f32 + math::cos(time)) * 0.5f32);
    gl::uniform_1f(texAlphaLoc as gl::GLint, u);

    let mat4TransLoc = pgrm.uniforms[UNIFORM_MAT4_TRANS];
    let rad          = time * 0.5f32 * pi();
    let trans        = mat3::from_angle_z(rad);
    // let v = vec3::new (1f32, 0f32, 1f32);
    // let v = Vector3::new(1f32, 0f32, 1f32);
    gl::ll::glUniformMatrix4fv(mat4TransLoc, 1, gl::FALSE as u8, trans.to_mat4().to_ptr());

    let model = &scene.models[0];
    gl::clear(gl::COLOR_BUFFER_BIT);
    gl::draw_elements_u8(gl::TRIANGLES, model.element_count as gl::GLsizei, None);
}