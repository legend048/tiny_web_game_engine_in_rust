fn main() {
    // let mut blue_amount = 0.0;

    // game_engine::set_event_handler(move |_key|{
    //     blue_amount += 0.1;
    //     game_engine::clear_screen_color(0.0, 0.0, blue_amount, 1.0);
    // });

    // game_engine::clear_screen_color(0.0, 0.0, 1.0, 1.0);

    let mut x_pos = 200.0;
    let mut y_pos = 30.0;

    
    let mut x_dir = 1.0;
    let mut y_dir = 1.0;

    let speed = 5.0;

    game_engine::set_event_handler(move |context ,event| match event {
        game_engine::Event::Draw => {
            x_pos += x_dir * speed;
            y_pos += y_dir * speed;

            if x_pos <= 0.0 || x_pos >= 500.0 {
                x_dir *= -1.0;
            }

            if y_pos <= 0.0 || y_pos >= 500.0 {
                y_dir *= -1.0;
            }

            context.clear_screen_color(0.0, 0.0, 0.3, 1.0);
            context.draw_rectangle(x_pos, y_pos, 100., 100.);
        }
        _ => {}
    });
}
