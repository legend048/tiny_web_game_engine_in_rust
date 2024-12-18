pub mod game;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;
// use std::cell::RefCell;


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

pub type EventCallback = Box<dyn Fn(&mut Context, Event)>;

pub enum Event {
    KeyDown(Key),
    MouseMove { x: f32, y: f32 },
    MouseClick { button: u8, x: f32, y: f32 },
    Draw,
}

pub enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

pub struct Game {
    pub state: GameState,
}

pub struct Timer {
    duration: f32,
    elapsed: f32,
}

pub static TARGET_FPS: RefCell<f64> = RefCell::new(60.0);


#[wasm_bindgen]
extern "C" {
    pub fn update_score(number: usize);
    pub fn change_screen_color(red: f32, green: f32, blue: f32, alpha: f32);
    pub fn js_draw_rectangle(x: f32, y: f32, width: f32, height: f32, red: f32, green: f32, blue: f32, alpha: f32);
    pub fn play_sound(src: &str);
    pub fn js_draw_sprite(x: f32, y: f32, width: f32, height: f32, src: &str);
    pub fn js_draw_text(text: &str, x: f32, y: f32, size: f32, r: f32, g: f32, b: f32, a: f32);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log_debug(s: &str);
    // log_debug(&format!("Rectangles to draw: {:?}", rectangles));
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


impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::MainMenu,
        }
    }

    pub fn update(&mut self, event: Event) {
        match self.state {
            GameState::MainMenu => {
                if let Event::KeyDown(Key::Space) = event {
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                // Update game logic
            }
            GameState::GameOver => {
                // Handle Game Over
            }
        }
    }
}


impl Timer {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        self.elapsed += delta_time;
        if self.elapsed >= self.duration {
            self.elapsed = 0.0;
            return true;
        }
        false
    }
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

            // log(&format!("Frame time: {:.2}", self.delta_time));
        }

        self.delta_time
    }
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
    let current_time_seconds = current_time / 1000.0; // Convert milliseconds to seconds

    EVENT_HANDLERS_AND_CONTEXT.with(|handlers_and_context| {
        let mut borrow = handlers_and_context.borrow_mut();
        let (_, context) = &mut *borrow;
        context.update_time(current_time_seconds); // Update the time in the context
    });

    // Dispatch the Draw event
    send_event(Event::Draw);
}


#[wasm_bindgen]
pub fn get_fps() -> f64 {
    EVENT_HANDLERS_AND_CONTEXT.with(|handlers_and_context| {
        let borrow = handlers_and_context.borrow();
        let (_, context) = &*borrow;
        context.fps // Return the FPS value
    })
}

#[wasm_bindgen]
pub fn set_target_fps(fps: f64) {
    TARGET_FPS.with(|target| *target.borrow_mut() = fps);
}
