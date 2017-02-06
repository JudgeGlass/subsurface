use glutin::{Event, ElementState, VirtualKeyCode, MouseButton};
use cgmath::{Vector3, vec3, Vector2, vec2};

pub enum Command {
    Noop,
    Exit,
    CameraTranslate(State, Vector3<f32>),
    CameraLook(Vector2<f32>),
    Place,
}

pub enum State {
    Start,
    Stop,
}

pub fn glutin_event_to_command(event: Event) -> Command {
    match event {
        Event::Closed => Command::Exit,
        Event::KeyboardInput(state, _, key) => glutin_key_to_command(state, key),
        Event::MouseMoved(x, y) => Command::CameraLook(vec2(x as f32, y as f32)),
        Event::MouseInput(ElementState::Pressed, MouseButton::Right) => Command::Place,
        _ => Command::Noop,
    }
}

fn glutin_key_to_command(state: ElementState, key: Option<VirtualKeyCode>) -> Command {
    match key {
        Some(VirtualKeyCode::Escape) => Command::Exit,
        Some(VirtualKeyCode::S) => {
            Command::CameraTranslate(glutin_state_to_state(state), vec3(-1.0, 0.0, 0.0))
        }
        Some(VirtualKeyCode::W) => {
            Command::CameraTranslate(glutin_state_to_state(state), vec3(1.0, 0.0, 0.0))
        }
        Some(VirtualKeyCode::D) => {
            Command::CameraTranslate(glutin_state_to_state(state), vec3(0.0, 0.0, 1.0))
        }
        Some(VirtualKeyCode::A) => {
            Command::CameraTranslate(glutin_state_to_state(state), vec3(0.0, 0.0, -1.0))
        }
        Some(VirtualKeyCode::Space) => {
            Command::CameraTranslate(glutin_state_to_state(state), vec3(0.0, 1.0, 0.0))
        }
        Some(VirtualKeyCode::LShift) => {
            Command::CameraTranslate(glutin_state_to_state(state), vec3(0.0, -1.0, 0.0))
        }
        _ => Command::Noop,
    }
}


fn glutin_state_to_state(state: ElementState) -> State {
    match state {
        ElementState::Pressed => State::Start,
        ElementState::Released => State::Stop,
    }
}
