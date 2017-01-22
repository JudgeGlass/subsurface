use cgmath;
pub use cgmath::{Vector3, Vector2, Matrix4, Point3, vec3, vec2};

pub type SVector3<I> = [I; 3];
pub type SMatrix4<I> = [[I; 4]; 4];

#[inline]
pub fn clamp<I: PartialOrd>(value: I, min: I, max: I) -> I {
    assert!(min <= max);
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[inline]
pub fn point3<I: cgmath::BaseNum>(x: I, y: I, z: I) -> Point3<I> {
    Point3::new(x, y, z)
}