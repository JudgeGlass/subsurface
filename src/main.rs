#[macro_use]
extern crate log;
extern crate glutin;
extern crate cgmath;
#[macro_use]
extern crate bitflags;
extern crate chrono;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate rustc_serialize;
extern crate bincode;
extern crate num_iter;
extern crate clap;
extern crate image;
extern crate noise;
extern crate num;

extern crate gfx_debug_draw;
extern crate gfx_text;

use std::time::Instant;
use std::f32;
use std::path::Path;

use cgmath::prelude::*;

mod graphics;
mod input;
mod prelude;
mod world;
mod logger;

use prelude::*;

use clap::{App, Arg};

fn main() {
    use gfx::Device;

    logger::init().unwrap();

    let matches = App::new("subsurface")
        .version("0.1.0")
        .about("Rust voxel engine")
        .arg(Arg::with_name("world")
            .help("Path to world directory to load")
            .long("world")
            .short("w")
            .takes_value(true)
            .default_value("test_world"))
        .arg(Arg::with_name("generator")
            .help("Use flat terrain generator")
            .long("generator")
            .short("g")
            .takes_value(true)
            .default_value("simplex"))
        .get_matches();

    let builder = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_stencil_buffer(8)
        .with_dimensions(1024, 768)
        .with_title("Subsurface");

    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<gfx::format::Srgba8, gfx::format::DepthStencil>(builder);

    window.set_cursor_state(glutin::CursorState::Grab)
        .unwrap();

    let mut voxrender =
        graphics::Renderer::new(&mut factory, main_color.clone(), main_depth.clone());
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut camera_frame_translator = vec3(0.0, 0.0, 0.0);
    let mut reference_time = Instant::now();

    let mut cycler: u64 = 0;

    let world_path = Path::new(matches.value_of("world").unwrap());
    let chunk_gen: Box<world::terrain::ChunkGenerator> = match matches.value_of("generator")
        .unwrap() {
        "flat" => Box::new(world::terrain::FlatGenerator::new(50, 1, "stone".into())),
        _ => Box::new(world::terrain::SimplexGenerator::new(50, 1)),
    };

    let mut world = world::World::from_path(&world_path,
                                            (vec3(-64, 0, -64), vec3(64, 64, 64)),
                                            chunk_gen);

    let mut models = world.make_models(&mut factory);
    for model in models.drain(..) {
        voxrender.set_model(model.0, model.1);
    }


    let text_renderer = gfx_text::new(factory.clone()).unwrap();
    let mut debug_renderer = gfx_debug_draw::DebugRenderer::new(factory.clone(), text_renderer, 64)
        .unwrap();

    info!("Starting main loop");
    loop {
        let new_time = Instant::now();
        let elapsed = new_time.duration_since(reference_time);
        reference_time = new_time;
        let delta = elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f32 / 1_000_000_000.0);

        if cycler % 1000 == 0 || (delta > 0.016 && cycler % 10 == 0) {
            info!("Frame time {}", (delta * 1000.0) as u64);
        }

        for command in window.poll_events().map(input::glutin_event_to_command) {
            match command {
                input::Command::Exit => return,
                input::Command::CameraTranslate(input::State::Start, ammount) => {
                    camera_frame_translator += ammount;
                    // Key-repeat is dumb
                    camera_frame_translator.x = clamp(camera_frame_translator.x, -1.0, 1.0);
                    camera_frame_translator.y = clamp(camera_frame_translator.y, -1.0, 1.0);
                    camera_frame_translator.z = clamp(camera_frame_translator.z, -1.0, 1.0);
                }
                input::Command::CameraTranslate(input::State::Stop, ammount) => {
                    camera_frame_translator -= ammount;
                }
                input::Command::CameraLook(vec) => {
                    let relative = vec - vec2(500.0, 500.0);
                    voxrender.camera.look_around(0.01 * relative);

                    window.set_cursor_position(500, 500)
                        .unwrap();
                }
                input::Command::Place => {
                    let direction = (voxrender.camera.look_at - voxrender.camera.position)
                        .normalize();
                    let casted = world.cast_ray(voxrender.camera.position, 20.0 * direction);
                    println!("Raycast result {:?}", casted);

                    if let Some((loc, face)) = casted {
                        let id = world.registry.lookup_id(&"stone".into()).unwrap();
                        world.place_block(loc + face.normal(), id);
                    }
                }
                input::Command::Save => {
                    world.write_all_chunks();
                }
                input::Command::Noop => (),
            }
        }

        // Update as much as one dirty chunk per frame
        world.clean_chunk(&mut factory).map(|o| voxrender.set_model(o.0, o.1));

        voxrender.camera.relative_translate(delta * 5.0 * camera_frame_translator);
        debug_renderer.draw_text_on_screen(&format!("Camera {:?}", voxrender.camera.position),
                                           [0, 0],
                                           [1.0, 1.0, 1.0, 1.0]);
        debug_renderer.draw_text_on_screen(&format!("Phi: {} Theta: {}",
                                                    voxrender.camera.phi,
                                                    voxrender.camera.theta),
                                           [0, 20],
                                           [1.0, 1.0, 1.0, 1.0]);

        voxrender.render(&mut encoder);

        debug_renderer.render(&mut encoder,
                    &main_color,
                    &main_depth,
                    (Matrix4::from(voxrender.projection) *
                     Matrix4::from(*voxrender.camera.get_view_matrix()))
                        .into())
            .unwrap();

        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();

        cycler += 1;
    }
}
