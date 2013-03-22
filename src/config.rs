#[inline(always)]
pub fn shader_path(file_name: &str) -> ~Path
{
    ~PosixPath(str::append(~"data/shaders/glsl/", file_name))
}