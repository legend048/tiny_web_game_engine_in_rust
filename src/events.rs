use std::collections::HashMap;
use crate::context::Context;
use wasm_bindgen::prelude::wasm_bindgen;

pub type EventCallback = Box<dyn Fn(&mut Context, Event)>;

pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
}

pub enum Event {
    KeyDown(Key),
    MouseMove { x: f32, y: f32 },
    MouseClick { button: u8, x: f32, y: f32 },
    Draw,
}

#[wasm_bindgen]
pub extern "C" fn key_pressed(value: usize) {
    let key = match value {
        1 => Key::Left,
        2 => Key::Right,
        3 => Key::Up,
        4 => Key::Down,
        5 => Key::Space,
        _ => return,
    };

    send_event(Event::KeyDown(key));
}

thread_local! {
    pub static EVENT_HANDLERS_AND_CONTEXT: std::cell::RefCell<(HashMap<String, Box<dyn FnMut(&mut Context, Event)>>, Context)> =
        std::cell::RefCell::new((
            HashMap::new(),
            Context { 
                last_time: 0.0,
                delta_time: 0.0,
                frame_count: 0,
                fps: 0.0,
                elapsed_time: 0.0,
            }
        ));
}

pub fn register_event_handler(event_name: &str, handler: impl FnMut(&mut Context, Event) + 'static) {
    EVENT_HANDLERS_AND_CONTEXT.with(|handlers_and_context| {
        let mut handlers = handlers_and_context.borrow_mut();
        handlers.0.insert(event_name.to_string(), Box::new(handler));
    });
}

pub fn send_event(event: Event) {
    let event_name = match &event {
        Event::KeyDown(_) => "KeyDown",
        Event::MouseMove { .. } => "MouseMove",
        Event::MouseClick { .. } => "MouseClick",
        Event::Draw => "Draw",
    };

    EVENT_HANDLERS_AND_CONTEXT.with(|handlers_and_context| {
        let mut handlers = handlers_and_context.borrow_mut();
        let (handlers_map, context) = &mut *handlers;

        if let Some(handler) = handlers_map.get_mut(event_name) {
            handler(context, event);
        }
    });
}
