use std::fmt::Display;



pub struct Vec2F {
    pub x: f32,
    pub y: f32,
}

impl Vec2F {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
}

impl Display for Vec2F {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}