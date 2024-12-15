use core::borrow;

pub struct Context {}

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

thread_local! {
    pub static EVENT_HANDLER_AND_CONTEXT: std::cell::RefCell<(Box<dyn FnMut(&mut Context, Event)>, Context)> = std::cell::RefCell::new((Box::new(|_, _|{}), Context {}));
}

pub fn set_event_handler(function: impl FnMut(&mut Context, Event) + 'static) {
    EVENT_HANDLER_AND_CONTEXT.with(|event_handler_and_context| {
        event_handler_and_context.borrow_mut().0 = Box::new(function);
    });
}

impl Context {
    pub fn clear_screen_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32){
        unsafe {
            change_screen_color(red, green, blue, alpha);
        }
    }


    pub fn draw_rectangle(&mut self ,x: f32, y: f32, width: f32, height: f32){
        unsafe {
            js_draw_rectangle(x, y, width, height);
        }
    }
}

fn send_event(event: Event) {
    EVENT_HANDLER_AND_CONTEXT.with(|event_handler_and_context|{
        let mut borrow = event_handler_and_context.borrow_mut();
        let (event_handler, context) = &mut *borrow;
        (event_handler)(context, event)
    })
}

#[no_mangle]
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

#[no_mangle]
pub extern "C" fn animate(){
    send_event(Event::Draw);
}
