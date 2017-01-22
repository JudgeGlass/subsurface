use glium;
use glium::{Program, Frame, Surface, DrawParameters};
use glium::backend::Facade;

use cgmath::{Matrix4, Point3, vec3, Deg, perspective};

use super::model::Model;

pub struct Renderer {
    program: Program,
    model: Model,
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
        let camera_position = Point3::new(-4.0, 7.0, -4.0);
        let view: [[f32; 4]; 4] = Matrix4::look_at(camera_position,
                                                   Point3::new(0.0, 0.0, 0.0),
                                                   vec3(0.0, 1.0, 0.0))
            .into();
        let projection: [[f32; 4]; 4] =
            perspective(Deg(90.0), width as f32 / height as f32, 0.1, 100.0).into();

        frame.draw(&self.model.vbo,
                  &self.model.ibo,
                  &self.program,
                  &uniform!(projection: projection, view: view, model: self.model.model),
                  &params)
            .unwrap();
    }
}
