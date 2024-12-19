use std::sync::RwLock;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{bindings::log, events::{send_event, Event, EVENT_HANDLERS_AND_CONTEXT}};

pub static TARGET_FPS: RwLock<f64> = RwLock::new(60.0);

#[wasm_bindgen]
pub extern "C" fn animate(current_time: f64) {
    let current_time_seconds = current_time / 1000.0;

    EVENT_HANDLERS_AND_CONTEXT.with(|handlers_and_context| {
        let mut borrow = handlers_and_context.borrow_mut();
        let (_, context) = &mut *borrow;
        context.update_time(current_time_seconds);
    });

    send_event(Event::Draw);
}

#[wasm_bindgen]
pub fn get_fps() -> f64 {
    EVENT_HANDLERS_AND_CONTEXT.with(|handlers_and_context| {
        let borrow = handlers_and_context.borrow();
        let (_, context) = &*borrow;
        context.fps
    })
}

#[wasm_bindgen]
pub fn set_target_fps(fps: f64) {
    if let Ok(mut target) = TARGET_FPS.write() {
        *target = fps;
    } else {
        log("Failed to acquire write lock for TARGET_FPS");
    }
}
