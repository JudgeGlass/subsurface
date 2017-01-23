use prelude::*;

use glium;
use glium::{Program, Frame, Surface, DrawParameters};
use glium::backend::Facade;

use cgmath::{Deg, perspective};

use super::model::Model;
use super::Camera;

pub struct Renderer {
    program: Program,
    models: Vec<Model>,
    pub camera: Camera,
}

impl Renderer {
    pub fn new<F: Facade>(display: &F) -> Renderer {
        let vertex_shader_src = include_str!("../../resources/vertex.glsl");
        let fragment_shader_src = include_str!("../../resources/fragment.glsl");
        let program = Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

        Renderer {
            program: program,
            models: Vec::new(),
            camera: Camera::new(point3(0.0, 7.0, 0.0), 0.0, 0.0),
        }
    }

    pub fn add_models(&mut self, mut models: Vec<Model>) {
        self.models.append(&mut models);
    }

    pub fn render(&self, frame: &mut Frame) {
        let params = DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        let (width, height) = frame.get_dimensions();
        let projection: [[f32; 4]; 4] =
            perspective(Deg(90.0), width as f32 / height as f32, 0.1, 1000.0).into();

        for model in self.models.iter() {
            frame.draw(&model.vbo,
                      &model.ibo,
                      &self.program,
                      &uniform!(
                           projection: projection,
                           view: *self.camera.get_view_matrix(),
                           model: model.model),
                      &params)
                .unwrap();
        }
    }
}
