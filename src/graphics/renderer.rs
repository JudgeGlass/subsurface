use glium;
use glium::{Program, Frame, Surface, DrawParameters};
use glium::backend::Facade;

use cgmath::{Point3, vec3, Deg, perspective};

use super::model::Model;
use super::Camera;

pub struct Renderer {
    program: Program,
    model: Model,
    pub camera: Camera,
}

impl Renderer {
    pub fn new<F: Facade>(display: &F) -> Renderer {
        let vertex_shader_src = include_str!("../../resources/vertex.glsl");
        let fragment_shader_src = include_str!("../../resources/fragment.glsl");
        let program = Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();
        let model = Model::new(display, vec3(0.0, 0.0, 0.0));

        Renderer {
            program: program,
            model: model,
            camera: Camera::new(Point3::new(-4.0, 7.0, -4.0), 0.0, 0.0),
        }
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
            perspective(Deg(90.0), width as f32 / height as f32, 0.1, 100.0).into();

        frame.draw(&self.model.vbo,
                  &self.model.ibo,
                  &self.program,
                  &uniform!(
                       projection: projection,
                       view: *self.camera.get_view_matrix(),
                       model: self.model.model),
                  &params)
            .unwrap();
    }
}
