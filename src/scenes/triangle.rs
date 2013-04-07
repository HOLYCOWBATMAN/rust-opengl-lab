use core::sys::size_of;

use config;
use glcore::*;
use scene;

pub fn init(width: i32, height: i32) -> ~scene::Scene
{
    // Create Vertex Array Object
    let mut vao: GLuint = 0;
    glGenVertexArrays(1, &vao);
    glBindVertexArray(vao);

    // Create a Vertex Buffer Object and copy the vertex data to it
    let vbo: GLuint = 0;
    glGenBuffers(1, &vbo);

    let vertices: [GLfloat, ..15] = [
         0.0,  0.5,   1.0, 0.0, 0.0,
         0.5, -0.5,   0.0, 1.0, 0.0,
        -0.5, -0.5,   0.0, 0.0, 1.0
    ];

    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    unsafe {
        glBufferData(GL_ARRAY_BUFFER,
                     (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
                     cast::transmute(&vertices[0]),
                     GL_STATIC_DRAW);
    }

    let shader_program: GLuint = glCreateProgram();

    if shader_program == 0
    {
        fail!(~"Program done failed to create");
    }
    else
    {
        // TODO(BH): research a for comprehension style handling of Result/Options as in Scala
        let frag_shdr = scene::attach_shader_from_file(shader_program, GL_FRAGMENT_SHADER, config::shader_path(~"unit.fs"));
        let vert_shdr = scene::attach_shader_from_file(shader_program, GL_VERTEX_SHADER, config::shader_path(~"unit.vs"));

        glBindFragDataLocation(shader_program, 0, str::as_c_str("outColor", |s|s));

        match scene::link_program(shader_program)
        {
            Ok(shader_program) => {
                glUseProgram(shader_program);

                let pos_attrib = glGetAttribLocation(shader_program, str::as_c_str("position", |s|s)) as GLuint;
                glEnableVertexAttribArray(pos_attrib);
                glVertexAttribPointer(pos_attrib, 2, GL_FLOAT, GL_FALSE,
                                      5 * sys::size_of::<GLfloat>() as GLsizei,
                                      ptr::null());

                let col_attrib = glGetAttribLocation(shader_program, str::as_c_str("color", |s|s)) as GLuint;
                glEnableVertexAttribArray(col_attrib);
                unsafe {
                    glVertexAttribPointer(col_attrib, 3, GL_FLOAT, GL_FALSE,
                                          5 * sys::size_of::<GLfloat>() as GLsizei,
                                          cast::transmute(2 * sys::size_of::<GLfloat>() as uint));
                }

                glViewport(0, 0, width, height);

                let program = scene::ShaderProgram {
                    id: shader_program,
                    shaders: ~[frag_shdr, vert_shdr],
                    uniforms: ~[]
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
    // Clear the screen to black
    glClearColor(0.1, 0.1, 0.1, 1.0);
    glClear(GL_COLOR_BUFFER_BIT);
    // Draw a triangle from the 3 vertices
    glDrawArrays(GL_TRIANGLES, 0, 3);
}