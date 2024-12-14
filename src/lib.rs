extern "C" {
    pub fn log_number(number: usize);
    fn change_screen_color(red: f32, green: f32, blue: f32, alpha: f32);
}

pub fn clear_screen_color(red: f32, green: f32, blue: f32, alpha: f32){
    unsafe {
        change_screen_color(red, green, blue, alpha);
    }
}

#[no_mangle]
pub extern "C" fn key_pressed() {
    clear_screen_color(0.0, 1.0, 0.0, 1.0);
}