#[macro_use]
extern crate glium;
extern crate glutin;
extern crate cgmath;

use cgmath::{Deg, Matrix4, Point3, Vector3, perspective, vec3};

mod model;

fn main() {
    use glium::DisplayBuild;
    use glium::Surface;

    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();

    let vertex_shader_src = include_str!("../resources/vertex.glsl");
    let fragment_shader_src = include_str!("../resources/fragment.glsl");

    let model = model::Model::new(&display, vec3(0.0, 0.0, 0.0));

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    loop {
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                _ => (),
            }
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let (width, height) = target.get_dimensions();
        let view: [[f32; 4]; 4] = Matrix4::look_at(Point3::new(-4.0, 7.0, -4.0),
                                                   Point3::new(0.0, 0.0, 0.0),
                                                   Vector3::new(0.0, 1.0, 0.0))
            .into();
        let projection: [[f32; 4]; 4] =
            perspective(Deg(90.0), width as f32 / height as f32, 0.1, 100.0).into();

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target.draw(&model.vbo,
                  &model.ibo,
                  &program,
                  &uniform!(projection: projection, view: view, model: model.model),
                  &params)
            .unwrap();
        target.finish().unwrap();
    }
}
