use core::sys::size_of;
use gl = opengles::gl3;
use scene;
use config;

pub fn init(width: i32, height: i32) -> ~scene::Scene
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
        let frag_shdr = scene::attach_shader_from_file(pgrm, gl::FRAGMENT_SHADER, config::shader_path(~"unit.fs"));
        let vert_shdr = scene::attach_shader_from_file(pgrm, gl::VERTEX_SHADER, config::shader_path(~"unit.vs"));

        gl::bind_frag_data_location(pgrm, 0, ~"outColor");

        match scene::link_program(pgrm)
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

                let program = scene::ShaderProgram {
                    id: pgrm,
                    shaders: ~[frag_shdr, vert_shdr],
                };

                let model = scene::Model {
                    buffers: ~[vbo],
                    vertex_arrays: ~[vao],
                    element_count: 0,
                    textures: ~[]
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

pub fn draw(_scene: &scene::Scene)
{
    gl::clear(gl::COLOR_BUFFER_BIT);
    gl::draw_arrays(gl::TRIANGLES, 0, 3);
}