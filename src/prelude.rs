use cgmath;
pub use cgmath::{Vector3, Vector2, Matrix4, Point3, vec3, vec2, Point2};

pub use gfx::format::{U8Norm, U16Norm};

pub type TransformMatrix = [[f32; 4]; 4];
pub type UV = [U16Norm; 2];

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

#[inline]
pub fn point2<I: cgmath::BaseNum>(x: I, y: I) -> Point2<I> {
    Point2::new(x, y)
}
