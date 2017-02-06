use cgmath::{Basis2, Rotation2, Rotation, Rad};

use prelude::*;
use std::f32;

pub struct Camera {
    pub phi: f32,
    pub theta: f32,
    pub position: Point3<f32>,
    pub look_at: Point3<f32>,

    view_matrix: TransformMatrix,
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

fn phi_theta_pos_to_matrix(phi: f32,
                           theta: f32,
                           position: Point3<f32>)
                           -> (TransformMatrix, Point3<f32>) {
    let look_at = phi_theta_to_focus(phi, theta, position);
    (Matrix4::look_at(position, look_at, phi_theta_to_up(phi, theta)).into(), look_at)
}

impl Camera {
    pub fn new(position: Point3<f32>, phi: f32, theta: f32) -> Camera {
        let (matrix, look_at) = phi_theta_pos_to_matrix(phi, theta, position);
        Camera {
            phi: phi,
            theta: theta,
            position: position,
            view_matrix: matrix,
            look_at: look_at,
        }
    }

    pub fn relative_translate(&mut self, offset: Vector3<f32>) {
        let rotated = Basis2::from_angle(Rad(self.phi)).rotate_vector(vec2(offset.x, offset.z));
        self.position += vec3(rotated.x, offset.y, rotated.y);
        self.recompute();
    }

    fn recompute(&mut self) {
        let (matrix, look_at) = phi_theta_pos_to_matrix(self.phi, self.theta, self.position);
        self.view_matrix = matrix;
        self.look_at = look_at;
    }

    pub fn look_around(&mut self, motion: Vector2<f32>) {
        self.phi += motion.x;
        self.theta += motion.y;

        self.phi %= 2.0 * f32::consts::PI;
        self.theta = clamp(self.theta, 0.02, f32::consts::PI - 0.01);

        self.recompute();
    }

    pub fn get_view_matrix(&self) -> &TransformMatrix {
        &self.view_matrix
    }
}
