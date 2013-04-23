use lmath::vec::*;
use lmath::mat::*;
use lmath::quat::*;

pub struct Camera
{
    rot: quat,
    eye: vec3
}

impl Camera
{
    pub fn look_at(eye: &vec3, target: &vec3, up: &vec3) -> Camera
    {
        let vdir  = target.sub_v(eye).normalize();
        let vup   = up.sub_v(&vdir.mul_t(up.dot(&vdir))).normalize();
        let vside = vdir.cross(&vup);

        Camera
        {
            rot: quat::from_axes(vside, vup, vdir.neg()),
            eye: vec3::new(eye.x, eye.y, eye.z)
        }
    }

    pub fn to_mat4(&self) -> mat4
    {
        let eyeInv   = self.rot.mul_v(&self.eye).neg();

        let mut view = self.rot.to_mat3().to_mat4();
        let mut eye  = view.col_mut(3);
        eye.x        = eyeInv.x;
        eye.y        = eyeInv.y;
        eye.z        = eyeInv.z;

        view
    }
}