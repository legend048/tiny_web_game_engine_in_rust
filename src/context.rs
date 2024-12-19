use crate::bindings::change_screen_color;
use crate::bindings::js_draw_rectangle;

pub struct Context {
    pub last_time: f64,
    pub delta_time: f64,
    pub frame_count: usize,
    pub fps: f64,
    pub elapsed_time: f64,
}

impl Context {
    pub fn clear_screen_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        change_screen_color(red, green, blue, alpha);
    }

    pub fn draw_rectangle(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    ) {
        js_draw_rectangle(x, y, width, height, red, green, blue, alpha);
    }

    pub fn update_time(&mut self, current_time: f64) -> f64 {
        self.delta_time = current_time - self.last_time;
        self.last_time = current_time;
        self.elapsed_time += self.delta_time;
        self.frame_count += 1;

        if self.elapsed_time >= 1.0 {
            self.fps = self.frame_count as f64 / self.elapsed_time;
            self.frame_count = 0;
            self.elapsed_time = 0.0;
        }

        self.delta_time
    }
}
