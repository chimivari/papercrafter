
#[derive(Debug)]
pub struct Vec3F {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3F {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }
}