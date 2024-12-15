fn main() {
    // let mut blue_amount = 0.0;

    // game_engine::set_event_handler(move |_key|{
    //     blue_amount += 0.1;
    //     game_engine::clear_screen_color(0.0, 0.0, blue_amount, 1.0);
    // });

    // game_engine::clear_screen_color(0.0, 0.0, 1.0, 1.0);

    let mut x_pos = 200.0;
    let mut y_pos = 30.0;

    game_engine::set_event_handler(move |key| {
        // match key {
        //     game_engine::Key::Left => game_engine::clear_screen_color(1.0, 0.0, 0.0, 1.0),
        //     game_engine::Key::Right => game_engine::clear_screen_color(0.0, 1.0, 0.0, 1.0),
        //     game_engine::Key::Up => game_engine::clear_screen_color(0.0, 0.0, 1.0, 1.0),
        //     game_engine::Key::Down => game_engine::clear_screen_color(0.0, 1.0, 1.0, 1.0),
        //     game_engine::Key::Space => game_engine::clear_screen_color(1.0, 1.0, 0.0, 1.0),
        // }
        
        let move_amount = 20.0;
        match key {
            game_engine::Key::Left => x_pos -= move_amount,
            game_engine::Key::Right => x_pos += move_amount,
            game_engine::Key::Up => y_pos += move_amount,
            game_engine::Key::Down => y_pos -= move_amount,
            game_engine::Key::Space => {},
        }

        game_engine::clear_screen_color(0.0, 0.0, 0.3, 1.0);
        game_engine::draw_rectangle(x_pos, y_pos, 100., 100.);
    });
}
