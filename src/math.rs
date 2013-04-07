use core::num::NumCast::from;
// use core::num::Zero::zero;
// use core::num::One::one;
use core::num::{NumCast, Zero, One};
use std::cmp::FuzzyEq;

use lmath::mat::*;
use numeric::float::Float;
use numeric::{tan};

pub fn perspective<T: Float + FuzzyEq<T> + Neg<T> + Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Zero + One + NumCast>(vfov: T, aspect: T, near: T, far: T) -> Mat4<T>
{
    let _1: T  = from(1);
    let _2: T  = from(2);

    let angle  = vfov / _2;
    let ymax   = near * tan(angle);
    let xmax   = ymax * aspect;

    let left   = -xmax;
    let right  = xmax;
    let bottom = -ymax;
    let top    = ymax;

    let c0r0   = (_2 * near) / (right - left);
    let c0r1   = from(0);
    let c0r2   = from(0);
    let c0r3   = from(0);

    let c1r0   = from(0);
    let c1r1   = (_2 * near) / (top - bottom);
    let c1r2   = from(0);
    let c1r3   = from(0);

    let c2r0   = (right + left) / (right - left);
    let c2r1   = (top + bottom) / (top - bottom);
    let c2r2   = -(far + near) / (far - near);
    let c2r3   = -_1;

    let c3r0   = from(0);
    let c3r1   = from(0);
    let c3r2   = -(_2 * far * near) / (far - near);
    let c3r3   = from(0);

    BaseMat4::new(c0r0, c0r1, c0r2, c0r3,
              c1r0, c1r1, c1r2, c1r3,
              c2r0, c2r1, c2r2, c2r3,
              c3r0, c3r1, c3r2, c3r3)
}