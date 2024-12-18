pub struct PhysicsObject {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub ax: f32,
    pub ay: f32,
}

impl PhysicsObject {
    pub fn update(&mut self, delta_time: f32) {
        self.vx += self.ax * delta_time;
        self.vy += self.ay * delta_time;
        self.x += self.vx * delta_time;
        self.y += self.vy * delta_time;
    }
}
