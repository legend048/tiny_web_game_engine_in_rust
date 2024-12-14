fn main() {
    let mut blue_amount = 0.0;

    game_engine::set_event_handler(move ||{
        blue_amount += 0.1;
        game_engine::clear_screen_color(0.0, 0.0, blue_amount, 1.0);
    });

    // game_engine::clear_screen_color(0.0, 0.0, 1.0, 1.0);
}
