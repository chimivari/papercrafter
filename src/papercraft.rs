use paper_face::PaperFace;
use crate::mesh::face::Face;

pub mod paper_face;

pub struct Papercraft {
    pub size_mm: (usize, usize),
    pub faces: Vec<PaperFace>,
}


impl Papercraft {
    pub fn new() -> Self {
        Self {
            size_mm: (210, 297),
            faces: vec![],
        }
    }

    pub fn add_face(&mut self, face: &Face) {
        self.faces.push(PaperFace::new(face));
    }
}
