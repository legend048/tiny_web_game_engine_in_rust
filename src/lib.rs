pub mod game;

use wasm_bindgen::prelude::*;


pub struct Context {
    pub last_time: f64,
    pub delta_time: f64,
    pub frame_count: usize,
    pub fps: f64,
    pub elapsed_time: f64,
}

pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
}

pub enum Event {
    KeyDown(Key),
    Draw,
}

#[wasm_bindgen]
extern "C" {
    pub fn update_score(number: usize);
    pub fn change_screen_color(red: f32, green: f32, blue: f32, alpha: f32);
    pub fn js_draw_rectangle(x: f32, y: f32, width: f32, height: f32, red: f32, green: f32, blue: f32, alpha: f32);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

thread_local! {
    pub static EVENT_HANDLER_AND_CONTEXT: std::cell::RefCell<(Box<dyn FnMut(&mut Context, Event)>, Context)> =
        std::cell::RefCell::new((Box::new(|_, _| {}), Context { 
            last_time: 0.0,
            delta_time: 0.0,
            frame_count: 0,
            fps: 0.0,
            elapsed_time: 0.0,
        }));
}

pub fn set_event_handler(function: impl FnMut(&mut Context, Event) + 'static) {
    EVENT_HANDLER_AND_CONTEXT.with(|event_handler_and_context| {
        event_handler_and_context.borrow_mut().0 = Box::new(function);
    });
}

impl Context {
    pub fn clear_screen_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        change_screen_color(red, green, blue, alpha);
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, red: f32, green: f32, blue: f32, alpha: f32) {
        js_draw_rectangle(x, y, width, height, red, green, blue, alpha);
    }

    pub fn update_time(&mut self, current_time: f64) -> f64 {
        self.delta_time = current_time - self.last_time; // Time between frames
        self.last_time = current_time;
        self.elapsed_time += self.delta_time; // Accumulate elapsed time
        self.frame_count += 1;

        // Calculate FPS every 1 second
        if self.elapsed_time >= 1.0 {
            self.fps = self.frame_count as f64 / self.elapsed_time;
            self.frame_count = 0;
            self.elapsed_time = 0.0;

            // log(&format!("Calculated FPS: {:.2}", self.fps));
        }

        self.delta_time
    }
}

pub fn send_event(event: Event) {
    EVENT_HANDLER_AND_CONTEXT.with(|event_handler_and_context| {
        let mut borrow = event_handler_and_context.borrow_mut();
        let (event_handler, context) = &mut *borrow;
        (event_handler)(context, event)
    });
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

#[wasm_bindgen]
pub extern "C" fn animate(current_time: f64) {
    let current_time_seconds = current_time / 1000.0;
    // log(&format!("Animate called at time: {:.4} seconds", current_time_seconds));
    EVENT_HANDLER_AND_CONTEXT.with(|ctx| {
        let mut borrow = ctx.borrow_mut();
        let (_, context) = &mut *borrow;
        context.update_time(current_time_seconds); // Pass time in seconds
    });
    
    // log(&format!("Frame time: {:.2} ms", self.delta_time * 1000.0));
    send_event(Event::Draw);
}

#[wasm_bindgen]
pub fn get_fps() -> f64 {
    EVENT_HANDLER_AND_CONTEXT.with(|ctx| {
        let borrow = ctx.borrow();
        let (_, context) = &*borrow;
        context.fps
    })
}
