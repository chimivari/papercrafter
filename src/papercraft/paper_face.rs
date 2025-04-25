use nalgebra::Vector2 as Point;

use crate::mesh::face::Face;


pub struct PaperFace {
    pub points: Vec<Point<f64>>,
}

impl PaperFace {
    pub fn new(face: &Face) -> Self {
        let mut points = vec![];
        for v in &face.vertices {
            points.push(v.remove_row(1));
        }
        Self {
            points,
        }
    }
}
