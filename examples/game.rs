fn main() {
    // let mut blue_amount = 0.0;

    // game_engine::set_event_handler(move |_key|{
    //     blue_amount += 0.1;
    //     game_engine::clear_screen_color(0.0, 0.0, blue_amount, 1.0);
    // });

    // game_engine::clear_screen_color(0.0, 0.0, 1.0, 1.0);

    game_engine::set_event_handler(move |key| match key {
        game_engine::Key::Left => game_engine::clear_screen_color(1.0, 0.0, 0.0, 1.0),
        game_engine::Key::Right => game_engine::clear_screen_color(0.0, 1.0, 0.0, 1.0),
        game_engine::Key::Up => game_engine::clear_screen_color(0.0, 0.0, 1.0, 1.0),
        game_engine::Key::Down => game_engine::clear_screen_color(0.0, 1.0, 1.0, 1.0),
        game_engine::Key::Space => game_engine::clear_screen_color(1.0, 1.0, 0.0, 1.0),
    });
}
