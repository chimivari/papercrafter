use nalgebra::Vector3 as Vertex;

use super::face::Face;


pub struct Neighbour {
    pub child: usize,
    pub p1: usize,
    pub p2: usize,
    pub p_futherest: usize,
    pub c_futherest: usize,
}

impl Neighbour {
    pub fn new(child_face: usize, p_futherest: usize, c_futherest: usize) -> Self {
        Self {
            child: child_face,
            p1: (p_futherest + 1) % 3,
            p2: (p_futherest + 2) % 3,
            p_futherest,
            c_futherest
        }
    }

    pub fn unfold_from_here(&mut self, face: &mut Face) {
        
    }
}