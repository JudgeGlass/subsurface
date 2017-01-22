#[macro_use]
extern crate glium;
extern crate glutin;
extern crate cgmath;

use std::time::Instant;
use std::f32;

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

    display.get_window()
        .expect("Could not get window")
        .set_cursor_state(glutin::CursorState::Grab)
        .unwrap();

    if display.get_opengl_version() < &glium::Version(glium::Api::Gl, 3, 3) {
        println!("Error: OpenGL 3.3 or later is required");
        return;
    }

    let mut voxrender = graphics::Renderer::new(&display);

    let mut camera_frame_translator = cgmath::vec3(0.0, 0.0, 0.0);
    let mut reference_time = Instant::now();

    let mut last_mouse_coords = cgmath::vec2(0.0, 0.0);

    let mut cycler: u64 = 0;

    loop {
        let new_time = Instant::now();
        let elapsed = new_time.duration_since(reference_time);
        reference_time = new_time;
        let delta = elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f32 / 1_000_000_000.0);

        if cycler % 1000 == 0 || (delta > 0.016 && cycler % 10 == 0) {
            println!("Frame time {}", (delta * 1000.0) as u64);
        }

        for command in display.poll_events().map(input::glutin_event_to_command) {
            match command {
                input::Command::Exit => return,
                input::Command::CameraTranslate(input::State::Start, ammount) => {
                    camera_frame_translator += ammount;
                    // Key-repeat is dumb
                    camera_frame_translator.x = camera_frame_translator.x.max(-1.0).min(1.0);
                    camera_frame_translator.y = camera_frame_translator.y.max(-1.0).min(1.0);
                    camera_frame_translator.z = camera_frame_translator.z.max(-1.0).min(1.0);
                }
                input::Command::CameraTranslate(input::State::Stop, ammount) => {
                    camera_frame_translator -= ammount;
                }
                input::Command::CameraLook(vec) => {
                    let relative = vec - last_mouse_coords;
                    last_mouse_coords = vec;
                    voxrender.camera.look_around(0.01 * relative)
                }
                input::Command::Noop => (),
            }
        }

        voxrender.camera.relative_translate(delta * camera_frame_translator);


        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        voxrender.render(&mut target);

        target.finish().unwrap();

        cycler += 1;
    }
}
