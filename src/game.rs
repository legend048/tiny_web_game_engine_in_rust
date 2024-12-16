use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::{log_number, send_event, set_event_handler, Event, Key, log};
// use crate::{set_event_handler, Event, Key, log, log_number};
use rand::Rng;

#[wasm_bindgen]
pub fn run_game() {
    let snake = Rc::new(RefCell::new(vec![(400.0, 300.0)])); // Initial snake position
    let direction = Rc::new(RefCell::new((1.0, 0.0)));       // Initial direction (moving right)
    let grid_size = 20.0;                                   // Size of each grid cell
    let canvas_width = 800.0;                               // Canvas width
    let canvas_height = 600.0;                              // Canvas height
    let food = Rc::new(RefCell::new(generate_food(canvas_width, canvas_height, grid_size))); // Initial food position
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
                context.clear_screen_color(0.0, 0.0, 0.0, 1.0); // Black background

                let mut snake = snake.borrow_mut();
                let dir = *direction.borrow();

                // Calculate new head position
                let (head_x, head_y) = snake[0];
                let mut new_head = (head_x + dir.0 * grid_size, head_y + dir.1 * grid_size);

                // Wrap around the canvas
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

                // Check if snake eats food
                let mut food_pos = food.borrow_mut();
                if (new_head.0 - food_pos.0).abs() < f64::EPSILON && (new_head.1 - food_pos.1).abs() < f64::EPSILON {
                    score += 1;
                    log_number(score);
                    *food_pos = generate_food(canvas_width, canvas_height, grid_size); // Generate new food
                } else {
                    snake.pop(); // Remove the tail if no food is eaten
                }

                // Add the new head to the front of the snake
                snake.insert(0, new_head);

                // Draw the snake
                for &(x, y) in &*snake {
                    context.draw_rectangle(
                        x as f32, 
                        y as f32, 
                        grid_size as f32, // Use the grid size for width
                        grid_size as f32, // Use the grid size for height
                        0.0, 1.0, 0.0, 1.0, // Snake color (Green)
                    );
                    
                }

                // Draw the food
                context.draw_rectangle(
                    food_pos.0 as f32, 
                    food_pos.1 as f32, 
                    grid_size as f32, 
                    grid_size as f32, 
                    1.0, 0.0, 0.0, 1.0, // Red color
                );

                // Debugging logs (optional)
                log(&format!("Snake position: {:?}, Food position: {:?}", *snake, *food_pos));
            }
        }
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
    send_event(Event::Draw); // Trigger draw event
}

#[wasm_bindgen(start)]
pub fn main() {
    run_game();
    start_animation_loop();
}

fn start_animation_loop() {
    use wasm_bindgen::JsCast;
    use web_sys::window;

    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new({
        let g = g.clone();
        move || {
            animate_frame(0.0);

            window()
                .unwrap()
                .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }
    }) as Box<dyn FnMut()>));

    window()
        .unwrap()
        .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}
