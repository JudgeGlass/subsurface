#[macro_use]
extern crate glium;
extern crate glutin;
extern crate cgmath;

mod graphics;
mod input;

fn main() {
    use glium::DisplayBuild;
    use glium::Surface;

    let display = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();

    if display.get_opengl_version() < &glium::Version(glium::Api::Gl, 3, 3) {
        println!("Error: OpenGL 3.3 or later is required");
        return;
    }

    let voxrender = graphics::Renderer::new(&display);

    loop {
        for command in display.poll_events().map(input::glutin_event_to_command) {
            match command {
                input::Command::Exit => return,
                input::Command::Noop => (),
            }
        }


        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        voxrender.render(&mut target);

        target.finish().unwrap();
    }
}
