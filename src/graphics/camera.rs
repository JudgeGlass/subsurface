use cgmath::{Basis2, Rotation2, Rotation, Rad};

use prelude::*;
use std::f32;

pub struct Camera {
    phi: f32,
    theta: f32,
    position: Point3<f32>,

    view_matrix: SMatrix4<f32>,
}

fn phi_theta_to_focus(phi: f32, theta: f32, position: Point3<f32>) -> Point3<f32> {
    position +
    vec3(theta.sin() * phi.cos(),
         theta.cos(),
         theta.sin() * phi.sin())
}

fn phi_theta_to_up(phi: f32, theta: f32) -> Vector3<f32> {
    let theta_prime = theta - f32::consts::PI / 2.0;
    vec3(theta_prime.sin() * phi.cos(),
         theta_prime.cos(),
         theta_prime.sin() * phi.sin())
}

fn phi_theta_pos_to_matrix(phi: f32, theta: f32, position: Point3<f32>) -> SMatrix4<f32> {
    Matrix4::look_at(position,
                     phi_theta_to_focus(phi, theta, position),
                     phi_theta_to_up(phi, theta))
        .into()
}

impl Camera {
    pub fn new(position: Point3<f32>, phi: f32, theta: f32) -> Camera {
        Camera {
            phi: phi,
            theta: theta,
            position: position,
            view_matrix: phi_theta_pos_to_matrix(phi, theta, position),
        }
    }

    pub fn relative_translate(&mut self, offset: Vector3<f32>) {
        let rotated = Basis2::from_angle(Rad(self.phi)).rotate_vector(vec2(offset.x, offset.z));
        self.position += vec3(rotated.x, offset.y, rotated.y);
        self.recompute();
    }

    fn recompute(&mut self) {
        self.view_matrix = phi_theta_pos_to_matrix(self.phi, self.theta, self.position);
    }

    pub fn look_around(&mut self, motion: Vector2<f32>) {
        self.phi += motion.x;
        self.theta += motion.y;

        self.phi = self.phi % (2.0 * f32::consts::PI);
        self.theta = clamp(self.theta, 0.02, f32::consts::PI - 0.01);

        self.recompute();
    }

    pub fn get_view_matrix(&self) -> &SMatrix4<f32> {
        &self.view_matrix
    }
}
