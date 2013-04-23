extern mod glcore;
extern mod glfw;
extern mod lmath;
extern mod numeric;
extern mod stb_image;
extern mod std;

pub mod camera;
pub mod config;
// mod input;
#[path = "io/mod.rs"] mod io;
pub mod main;
pub mod math;
pub mod scene;
#[path = "scenes/mod.rs"] pub mod scenes;
pub mod screen;
// pub mod util;
