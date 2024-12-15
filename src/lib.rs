pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space
}

pub enum Event{
    KeyDown(Key),
    Draw,
}

extern "C" {
    pub fn log_number(number: usize);
    fn change_screen_color(red: f32, green: f32, blue: f32, alpha: f32);
    fn js_draw_rectangle(x: f32, y: f32, width: f32, height: f32);
}

pub fn clear_screen_color(red: f32, green: f32, blue: f32, alpha: f32){
    unsafe {
        change_screen_color(red, green, blue, alpha);
    }
}

thread_local! {
    pub static EVENT_HANDLER: std::cell::RefCell<Box<dyn FnMut(Event)>> = std::cell::RefCell::new(Box::new(|_|{}));
}

pub fn set_event_handler(function: impl FnMut(Event) + 'static) {
    EVENT_HANDLER.with(|event_handler| {
        *event_handler.borrow_mut() = Box::new(function);
    });
}

#[no_mangle]
pub extern "C" fn key_pressed(value: usize) {
    // clear_screen_color(0.0, 1.0, 0.0, 1.0);

    let key = match value {
        1 => Key::Left,
        2 => Key::Right,
        3 => Key::Up,
        4 => Key::Down,
        5 => Key::Space,
        _ => return,
    };

    EVENT_HANDLER.with(|event_handler| (event_handler.borrow_mut())(Event::KeyDown(key)))
}


pub fn draw_rectangle(x: f32, y: f32, width: f32, height: f32){
    unsafe {
        js_draw_rectangle(x, y, width, height);
    }
}

#[no_mangle]
pub extern "C" fn animate(){
    EVENT_HANDLER.with(|event_handler| (event_handler.borrow_mut())(Event::Draw))
}