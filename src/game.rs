use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::{*};
use rand::Rng;

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

    set_event_handler({
        let snake = Rc::clone(&snake);
        let direction = Rc::clone(&direction);
        let food = Rc::clone(&food);

        move |context, event| match event {
            Event::KeyDown(key) => {
                let mut dir = direction.borrow_mut();
                *dir = match key {
                    Key::Up if *dir != (0.0, 1.0) => (0.0, -1.0),
                    Key::Down if *dir != (0.0, -1.0) => (0.0, 1.0),
                    Key::Left if *dir != (1.0, 0.0) => (-1.0, 0.0),
                    Key::Right if *dir != (-1.0, 0.0) => (1.0, 0.0),
                    _ => *dir,
                };
            }
            Event::Draw => {
                // log(&format!("FPS: {:.2}", context.fps));

                context.clear_screen_color(0.0, 0.0, 0.0, 1.0);

                let mut snake = snake.borrow_mut();
                let dir = *direction.borrow();

                let (head_x, head_y) = snake[0];
                let mut new_head = (head_x + dir.0 * grid_size, head_y + dir.1 * grid_size);

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

                let mut food_pos = food.borrow_mut();
                if (new_head.0 - food_pos.0).abs() < f64::EPSILON && (new_head.1 - food_pos.1).abs() < f64::EPSILON {
                    score += 1;
                    update_score(score);
                    *food_pos = generate_food(canvas_width, canvas_height, grid_size);
                } else {
                    snake.pop();
                }

                snake.insert(0, new_head);

                for &(x, y) in &*snake {
                    context.draw_rectangle(
                        x as f32, 
                        y as f32, 
                        grid_size as f32, 
                        grid_size as f32, 
                        0.0, 1.0, 0.0, 1.0, 
                    );
                }

                context.draw_rectangle(
                    food_pos.0 as f32, 
                    food_pos.1 as f32, 
                    grid_size as f32, 
                    grid_size as f32, 
                    1.0, 0.0, 0.0, 1.0,
                );
            }
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
    let mut rng = rand::thread_rng();
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
    run_game(); // Start the game logic
    start_animation_loop(); // Start the animation loop
}

fn start_animation_loop() {
    use wasm_bindgen::JsCast;
    use web_sys::window;

    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    *f.borrow_mut() = Some(Closure::wrap(Box::new(move |current_time: f64| {
        // Clone the Rc<RefCell> for the next animation frame
        let g_clone = Rc::clone(&g);

        // Call the Rust animate function with the current time
        crate::animate(current_time);

        // Request the next animation frame
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