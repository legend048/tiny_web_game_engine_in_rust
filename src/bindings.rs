use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn update_score(number: usize);
    pub fn change_screen_color(red: f32, green: f32, blue: f32, alpha: f32);
    pub fn js_draw_rectangle(x: f32, y: f32, width: f32, height: f32, red: f32, green: f32, blue: f32, alpha: f32);
    pub fn play_sound(src: &str);
    pub fn js_draw_sprite(x: f32, y: f32, width: f32, height: f32, src: &str);
    pub fn js_draw_text(text: &str, x: f32, y: f32, size: f32, r: f32, g: f32, b: f32, a: f32);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log_debug(s: &str);
}
