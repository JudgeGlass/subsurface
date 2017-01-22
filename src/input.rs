use glutin::{Event, ElementState, VirtualKeyCode};

pub enum Command {
    Noop,
    Exit,
}

pub fn glutin_event_to_command(event: Event) -> Command {
    match event {
        Event::Closed => Command::Exit,
        Event::KeyboardInput(state, _, key) => glutin_key_to_command(state, key),
        _ => Command::Noop
    }
}

fn glutin_key_to_command(_: ElementState, key: Option<VirtualKeyCode>) -> Command {
    match key {
        Some(VirtualKeyCode::Escape) => Command::Exit,
        _ => Command::Noop
    }
}
