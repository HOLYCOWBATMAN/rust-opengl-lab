// use std::cmp::FuzzyEq;
// use core::num::{NumCast, One, Zero, cast};
// // use core::util::swap;

// // pub struct Foo<T> {
// //     a: T
// // }

// // trait Matrix<T, V>
// // {
// //     fn col_mut(&mut self, i: uint) -> &self/mut Foo<T>;
// //     fn swap_cols(&mut self, a: uint, b: uint);
// // }

// // pub struct Mat<T>
// // {
// //     x: Foo<T>,
// //     y: Foo<T>
// // }

// // impl <T> Matrix<T, Foo<T>> for Mat<T> {
// //     fn col_mut(&self, i: uint) -> &Foo<T> {
// //         match i {
// //             0 => &self.x,
// //             1 => &self.y,
// //             _ => fail!(fmt!("index out of bounds: expected an index from 0 to 1, but found %u", i))
// //         }
// //     }

// //     #[inline(always)]
// //     fn swap_cols(&mut self, _a: uint, _b: uint) {
// //         // let ac = self.col_mut(a);
// //         // let bc = self.col_mut(b);

// //         let ac = &mut self.x;
// //         let bc = &mut self.y;

// //         swap(ac, bc);
// //         // swap(&self/mut self.x, &self/mut self.y);
// //     }
// // }

// trait Length<T> {
//     fn len(&self) -> T;
// }

// trait LengthSqr<T> {
//     fn len_sqr(&self) -> T;
// }

// trait Normalise<T> : Length<T> {
//     fn norm(&self) -> Self;
// }

// pub fn vec3<T: Add<T,T> + Copy + FuzzyEq<T> + Mul<T,T> + One + Zero>
//     (x: T, y: T, z: T) -> vec3<T> {
//     vec3 {x: x, y: y, z: z}
// }

// pub struct vec3<T> {
//     x: T,
//     y: T,
//     z: T
// }

// // pub const ZeroMoo: Moo<u8> = Moo { x: 0u8 };
// // pub const ZeroV: &vec3<T> = &vec3<T> { x: Zero::zero(), y: Zero::zero(), z: Zero::zero() };
// // pub const ZeroV: vec3<f32> = vec3(Zero::zero(), Zero::zero(), Zero::zero());
// // const One: vec3<T>  = vec3<T> { x: One::one(), y: One::one(), z: One::one() };
// // const XAxis:vec3<T> = vec3 { x: 1., y: 0., z: 0. };
// // const YAxis:vec3<T> = vec3 { x: 0., y: 1., z: 0. };
// // const ZAxis:vec3<T> = vec3 { x: 0., y: 0., z: 1. };

// pub impl <T: Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + Neg<T> + FuzzyEq<T>> vec3<T> {
//     pure fn fuzzy_eq(&self, other: &vec3<T>) -> bool {
//         self.x.fuzzy_eq(&other.x) &&
//         self.y.fuzzy_eq(&other.y) &&
//         self.z.fuzzy_eq(&other.z)
//     }

//     pure fn add(&self, rhs: &vec3<T>) -> vec3<T> {
//         vec3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
//     }

//     pure fn sub(&self, rhs: &vec3<T>) -> vec3<T> {
//         vec3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
//     }

//     pure fn mul(&self, rhs: &T) -> vec3<T> {
//         vec3 {x: self.x * (*rhs), y: self.y * (*rhs), z: self.z * (*rhs)}
//     }

//     pure fn div(&self, rhs: &T) -> vec3<T> {
//         vec3 {x: self.x / (*rhs), y: self.y / (*rhs), z: self.z / (*rhs)}
//     }

//     pure fn neg(&self) -> vec3<T> {
//         vec3 {x: -self.x, y: -self.y, z: -self.z}
//     }

//     pure fn mul_inner(&self, b: vec3<T>) -> T {
//         self.x * b.x + self.y * b.y + self.z * b.z
//     }

//     pure fn mul_cross(&self, b: vec3<T>) -> vec3<T> {
//         vec3 {
//             x: self.y * b.z - self.z * b.y,
//             y: self.z * b.x - self.x * b.z,
//             z: self.x * b.y - self.y * b.x
//         }
//     }
// }

// // impl<T: Add<T,T> + Sub<T,T> + Mul<T,T> + Div<T,T> + FuzzyEq<T> + One + Zero> Normalise<T> for vec3<T> {
// impl<T: Length<T> + Zero> Normalise<T> for vec3<T> {
//     pure fn norm(&self) -> vec3<T> {
//         // let l = self.len();
//         // self * One::one()
//         vec3{x: Zero::zero(), y: Zero::zero(), z: Zero::zero()}
// //         // if l.fuzzy_eq(&Zero::zero())
// //         // {
// //         //     vec3{x: Zero::zero(), y: Zero::zero(), z: Zero::zero()}
// //         // }
// //         // else
// //         // {
// //         //     let _1: T = One::one();
// //         //     let k = _1 / l;
// //         //     self * k
// //         // }
//     }
// }

// impl<T: Add<T,T> + Mul<T,T>> LengthSqr<T> for vec3<T> {
//     pure fn len_sqr(&self) -> T {
//         self.x * self.x + self.y * self.y + self.z * self.z
//     }
// }

// impl Length<f32> for vec3<f32> {
//     pure fn len(&self) -> f32 {
//         f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
//     }
// }

// impl Length<f64> for vec3<f64> {
//     pure fn len(&self) -> f64 {
//         f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
//     }
// }

// #[test]
// fn add() {
//     let a = Zero;
//     let b = Zero;
//     fail_unless!(fuzzy_eq(a + b, 0))
// }