extern crate glium;
extern crate glutin;

fn main() {
    use glium::DisplayBuild;
    use glium::Surface;

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Hello world"))
        .build_glium()
        .unwrap();

    loop {
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                _ => (),
            }
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();
    }
}
