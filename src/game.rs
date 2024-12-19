use crate::events::Event;
use crate::events::Key;

pub enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

pub struct Game {
    pub state: GameState,
}

#[derive(Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::MainMenu,
        }
    }

    pub fn update(&mut self, event: Event) {
        match self.state {
            GameState::MainMenu => {
                if let Event::KeyDown(Key::Space) = event {
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                // Update game logic
            }
            GameState::GameOver => {
                // Handle Game Over
            }
        }
    }
}
