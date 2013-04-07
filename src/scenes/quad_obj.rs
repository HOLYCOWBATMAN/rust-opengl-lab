use config;
use core::sys::size_of;
use gl = opengles::gl3;
use imageio = stb_image::image;
// use io::model::obj_model_from_file;
// use io::translate::model_to_flat_data;
use scene;
// use util::println;

pub fn init(width: i32, height: i32) -> ~scene::Scene
{
    // let model_path = ~"data/models/quad/quad.obj";
    // let model_path = ~"data/models/banana/Banana.obj";
    // let model_path = ~"data/models/cube/cube.obj";

    // Err(err) => fail!(fmt!("failed to read file: %s", err))

    // match obj_model_from_file(model_path)
    // {
    //     Ok(mdl) => mdl
    // }

    // let (vertices, elements) = model_to_flat_data();
    let vertices: ~[gl::GLfloat] = ~[];
    let elements: ~[gl::GLuint] = ~[];

    // Create Vertex Array Object
    let vao: gl::GLuint = gl::gen_vertex_arrays(1)[0];
    gl::bind_vertex_array(vao);

    // Create a Vertex Buffer Object and copy the vertex data to it
    let vbo: gl::GLuint = gl::gen_buffers(1)[0];

    gl::bind_buffer(gl::ARRAY_BUFFER, vbo);
    gl::buffer_data(gl::ARRAY_BUFFER, vertices, gl::STATIC_DRAW);

    // Create an element array
    let ebo: gl::GLuint = gl::gen_buffers(1)[0];

    gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::buffer_data(gl::ELEMENT_ARRAY_BUFFER, elements, gl::STATIC_DRAW);

    let pgrm = gl::create_program();

    if pgrm == 0
    {
        fail!(~"Program done failed to create");
    }
    else
    {
        let frag_shdr = scene::attach_shader_from_file(pgrm, gl::FRAGMENT_SHADER, config::shader_path(~"ucolor.fs"));
        let vert_shdr = scene::attach_shader_from_file(pgrm, gl::VERTEX_SHADER, config::shader_path(~"ucolor.vs"));

        gl::bind_frag_data_location(pgrm, 0, ~"outColor");

        match scene::link_program(pgrm)
        {
            Ok(pgrm) => {
                gl::use_program(pgrm);

                let stride       = 5 * size_of::<gl::GLfloat>() as gl::GLsizei;
                let tex_offset   = 3 * size_of::<gl::GLfloat>() as gl::GLuint;

                // Specify the layout of the vertex data
                let posAttrib = gl::get_attrib_location(pgrm, ~"position");
                gl::enable_vertex_attrib_array(posAttrib);
                gl::vertex_attrib_pointer_f32(posAttrib, 3, false, stride, 0);

                let texAttrib = gl::get_attrib_location(pgrm, ~"texcoord");
                gl::enable_vertex_attrib_array(texAttrib);
                gl::vertex_attrib_pointer_f32(texAttrib, 2, false, stride, tex_offset);

                let uniColor = gl::get_uniform_location(pgrm, ~"inColor");
                gl::uniform_3f(uniColor, 1.0, 1.0, 1.0);

                let tex: gl::GLuint = gl::gen_textures(1)[0];

                // let image_path = ~"data/models/banana/Banana.png";
                let image_path = ~"data/models/quad/huis1.png";

                match imageio::load_with_depth(image_path, 3, false)
                {
                    imageio::Error => fail!(~"error loading image"),
                    imageio::ImageF32(_) => fail!(~"error: F32 image format is not supported"),
                    imageio::ImageU8(img) => {

                        let d: &[u8] = img.data;
                        let tex_trg = gl::TEXTURE_2D;

                        gl::tex_image_2d(
                            tex_trg,
                            0,
                            gl::RGB8 as gl::GLint,
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

                gl::clear_color(0.1f32, 0.1f32, 0.1f32, 1f32);
                gl::viewport(0, 0, width, height);

                let program = scene::ShaderProgram {
                    id: pgrm,
                    shaders: ~[frag_shdr, vert_shdr],
                    uniforms: ~[]
                };

                let model = scene::Model {
                    buffers: ~[vbo],
                    vertex_arrays: ~[vao],
                    element_count: elements.len(),
                    textures: ~[tex]
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

pub fn draw(scene: &scene::Scene)
{
    gl::clear(gl::COLOR_BUFFER_BIT);
    gl::draw_elements_u8(gl::TRIANGLES, scene.models[0].element_count as gl::GLsizei, None);
}