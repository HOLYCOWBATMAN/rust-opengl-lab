// use core::num::NumCast::from;
// use core::num::{NumCast, Zero, One};
// use std::cmp::FuzzyEq;

// use lmath::mat::*;
// use numeric::float::Float;
// use numeric::{tan};

// // pub fn new_perspective_projection(fov: Component, aspect_ratio: Component, near: Component, far: Component) -> Mat4x4
// //   {
// //     let mut mat = Mat4x4::new();

// //     let rad = (3.1415 * fov) / 180.0;
// //     let range = f32::tan(rad / 2.0) * near;
// //     let left = -range * aspect_ratio;
// //     let right = range * aspect_ratio;
// //     let bottom = -range;
// //     let top = range;

// //     mat.data[0][0] = (2.0 * near) / (right - left); mat.data[1][0] = 0.0; mat.data[2][0] = 0.0; mat.data[3][0] = 0.0;
// //     mat.data[0][1] = 0.0; mat.data[1][1] = (2.0 * near) / (top - bottom); mat.data[2][1] = 0.0; mat.data[3][1] = 0.0;
// //     mat.data[0][2] = 0.0; mat.data[1][2] = 0.0; mat.data[2][2] = -(far + near) / (far - near); mat.data[3][2] = -(2.0 * far * near) / (far - near);
// //     mat.data[0][3] = 0.0; mat.data[1][3] = 0.0; mat.data[2][3] = -1.0; mat.data[3][3] = 0.0;

// //     mat
// //   }

// // pub fn new_lookat(position: Vec3f, target: Vec3f, up: Vec3f) -> Mat4x4
// // {
// //     let mut forward = target - position;
// //     forward.normalize();

// //     let mut side = forward.cross(&up);
// //     side.normalize();

// //     let mut proper_up = side.cross(&forward);
// //     proper_up.normalize();

// //     let mut mat = Mat4x4::new();
// //     mat.data[0][0] = side.x;      mat.data[1][0] = side.y;      mat.data[2][0] = side.z;      mat.data[3][0] = -side.dot(&position);
// //     mat.data[0][1] = proper_up.x; mat.data[1][1] = proper_up.y; mat.data[2][1] = proper_up.z; mat.data[3][1] = -proper_up.dot(&position);
// //     mat.data[0][2] = -forward.x;  mat.data[1][2] = -forward.y;  mat.data[2][2] = -forward.z;  mat.data[3][2] = forward.dot(&position);
// //     mat.data[0][3] = 0.0;         mat.data[1][3] = 0.0;         mat.data[2][3] = 0.0;         mat.data[3][3] = 1.0;

// //     mat
// // }

// // pub fn new_lookat(position: Vec3f, target: Vec3f, up: Vec3f) -> Mat4<T>
// // {
// //     let mut forward = target - position;
// //     forward.normalize();

// //     let mut side = forward.cross(&up);
// //     side.normalize();

// //     let mut proper_up = side.cross(&forward);
// //     proper_up.normalize();

// //     let mut mat = Mat4x4::new();
// //     mat.data[0][0] = side.x;      mat.data[1][0] = side.y;      mat.data[2][0] = side.z;      mat.data[3][0] = -side.dot(&position);
// //     mat.data[0][1] = proper_up.x; mat.data[1][1] = proper_up.y; mat.data[2][1] = proper_up.z; mat.data[3][1] = -proper_up.dot(&position);
// //     mat.data[0][2] = -forward.x;  mat.data[1][2] = -forward.y;  mat.data[2][2] = -forward.z;  mat.data[3][2] = forward.dot(&position);
// //     mat.data[0][3] = 0.0;         mat.data[1][3] = 0.0;         mat.data[2][3] = 0.0;         mat.data[3][3] = 1.0;

// //     mat
// // }

// pub fn perspective<T: Float + FuzzyEq<T> + Neg<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Zero + One + NumCast>(vfov: T, aspect: T, near: T, far: T) -> Mat4<T>
// {
//     let pi =
//     let vfov_rad = (from(3.1415) * vfov) / from(180.0);
//     // let range = f32::tan(rad / 2.0) * near;
//     // let left = -range * aspect_ratio;
//     // let right = range * aspect_ratio;
//     // let bottom = -range;
//     // let top = range;

//     let _1: T  = from(1);
//     let _2: T  = from(2);

//     let angle  = vfov_rad / _2;
//     let ymax   = near * tan(angle);
//     let xmax   = ymax * aspect;

//     let left   = -xmax;
//     let right  = xmax;
//     let bottom = -ymax;
//     let top    = ymax;

//     let c0r0   = (_2 * near) / (right - left);
//     let c0r1   = from(0);
//     let c0r2   = from(0);
//     let c0r3   = from(0);

//     let c1r0   = from(0);
//     let c1r1   = (_2 * near) / (top - bottom);
//     let c1r2   = from(0);
//     let c1r3   = from(0);

//     let c2r0   = (right + left) / (right - left);
//     let c2r1   = (top + bottom) / (top - bottom);
//     let c2r2   = -(far + near) / (far - near);
//     let c2r3   = -_1;

//     let c3r0   = from(0);
//     let c3r1   = from(0);
//     let c3r2   = -(_2 * far * near) / (far - near);
//     let c3r3   = from(0);

//     BaseMat4::new(c0r0, c0r1, c0r2, c0r3,
//               c1r0, c1r1, c1r2, c1r3,
//               c2r0, c2r1, c2r2, c2r3,
//               c3r0, c3r1, c3r2, c3r3)
// }