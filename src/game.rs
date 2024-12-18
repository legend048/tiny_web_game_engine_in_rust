use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::{*};
use rand::Rng;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize, Debug)]
pub struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}


thread_local! {
    pub static SPEED: RefCell<f64> = RefCell::new(1.0);
}

#[wasm_bindgen]
pub fn run_game() {
    let snake = Rc::new(RefCell::new(vec![(400.0, 300.0)]));
    let direction = Rc::new(RefCell::new((1.0, 0.0)));
    let grid_size = 20.0;
    let canvas_width = 800.0;
    let canvas_height = 600.0;
    let food = Rc::new(RefCell::new(generate_food(canvas_width, canvas_height, grid_size)));
    let mut score = 0;

    // Register KeyDown handler
    register_event_handler("KeyDown", {
        let direction = Rc::clone(&direction);
        move |_context, event| {
            if let Event::KeyDown(key) = event {
                let mut dir = direction.borrow_mut();
                *dir = match key {
                    Key::Up if *dir != (0.0, 1.0) => (0.0, -1.0),
                    Key::Down if *dir != (0.0, -1.0) => (0.0, 1.0),
                    Key::Left if *dir != (1.0, 0.0) => (-1.0, 0.0),
                    Key::Right if *dir != (-1.0, 0.0) => (1.0, 0.0),
                    _ => *dir,
                };
            }
        }
    });

    // Register Draw handler
    register_event_handler("Draw", {
        let snake = Rc::clone(&snake);
        let direction = Rc::clone(&direction);
        let food = Rc::clone(&food);
        move |context, _event| {
            context.clear_screen_color(0.0, 0.0, 0.0, 1.0);

            let mut snake = snake.borrow_mut();
            let dir = *direction.borrow();

            let (head_x, head_y) = snake[0];
            let mut new_head = (head_x + dir.0 * grid_size, head_y + dir.1 * grid_size);

            // Wrap around the screen
            if new_head.0 < 0.0 {
                new_head.0 = canvas_width - grid_size;
            } else if new_head.0 >= canvas_width {
                new_head.0 = 0.0;
            }

            if new_head.1 < 0.0 {
                new_head.1 = canvas_height - grid_size;
            } else if new_head.1 >= canvas_height {
                new_head.1 = 0.0;
            }

            // Check if the snake eats the food
            let mut food_pos = food.borrow_mut();
            if (new_head.0 - food_pos.0).abs() < f64::EPSILON && (new_head.1 - food_pos.1).abs() < f64::EPSILON {
                score += 1;
                update_score(score);
                *food_pos = generate_food(canvas_width, canvas_height, grid_size);
            } else {
                snake.pop();
            }

            snake.insert(0, new_head);

            // Prepare rectangles for rendering
            let mut rectangles = vec![];

            // Add snake rectangles
            for &(x, y) in &*snake {
                rectangles.push(Rectangle {
                    x: x as f32,
                    y: y as f32,
                    width: grid_size as f32,
                    height: grid_size as f32,
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                });
            }

            // Add food rectangle
            rectangles.push(Rectangle {
                x: food_pos.0 as f32,
                y: food_pos.1 as f32,
                width: grid_size as f32,
                height: grid_size as f32,
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            });

            // Send rectangles to JavaScript for rendering
            let js_value = serde_wasm_bindgen::to_value(&rectangles).unwrap();
            js_sys::Reflect::set(
                &js_sys::global(),
                &"batched_rectangles".into(),
                &js_value,
            ).unwrap();
        }
    });
}



#[wasm_bindgen]
pub fn update_speed(new_speed: f64) {
    SPEED.with(|speed| {
        *speed.borrow_mut() = new_speed;
    });
}

fn generate_food(canvas_width: f64, canvas_height: f64, grid_size: f64) -> (f64, f64) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let x = rng.gen_range(0..(canvas_width as usize / grid_size as usize)) as f64 * grid_size;
    let y = rng.gen_range(0..(canvas_height as usize / grid_size as usize)) as f64 * grid_size;
    (x, y)
}



#[wasm_bindgen]
pub fn animate_frame(_current_time: f64) {
    send_event(Event::Draw);
}

#[wasm_bindgen(start)]
pub fn main() {
    run_game();
    start_animation_loop();
}

fn start_animation_loop() {
    use wasm_bindgen::JsCast;
    use web_sys::window;

    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    *f.borrow_mut() = Some(Closure::wrap(Box::new(move |current_time: f64| {
        let g_clone = Rc::clone(&g);

        crate::animate(current_time);

        window()
            .unwrap()
            .request_animation_frame(g_clone.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }) as Box<dyn FnMut(f64)>));

    window()
        .unwrap()
        .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}
