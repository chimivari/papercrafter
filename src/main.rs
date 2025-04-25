use nalgebra::{Vector2, Vector3 as Vertex};

use mesh::Mesh;
use triangle::Triangle;

mod mesh;
mod triangle;
mod papercraft;

fn main() {
    
}


#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use rand::Rng;

    use super::*;

    fn random_vertex(rng: &mut rand::rngs::ThreadRng) -> Vertex<f64> {
        Vertex::new(rng.random_range(-10.0..10.0), rng.random_range(-10.0..10.0), rng.random_range(-10.0..10.0))
    }

    fn random_point(rng: &mut rand::rngs::ThreadRng) -> Vertex<f64> {
        Vertex::new(rng.random_range(-10.0..10.0), 0., rng.random_range(-10.0..10.0))
    }

    // #[test]
    fn test_align() {
        let mut rng = rand::rng();
        let vertices = vec![
            random_vertex(&mut rng),
            random_vertex(&mut rng),
            random_vertex(&mut rng),
            random_vertex(&mut rng),
        ];
        let triangles = vec![
            Triangle(0, 1, 2),
            Triangle(0, 1, 3),
        ];
        let mut mesh = Mesh::new(&triangles, &vertices);
        mesh.flatten(0);
        mesh.align(1, mesh.faces[0].neighbourhoud[0]);
        assert!(mesh.faces[0].is_flat(), "Face 0 is not flat, {mesh}");
        assert!(mesh.faces[1].is_flat(), "Face 1 is not flat, {mesh}");
        println!("{mesh}");
    }

    // #[test]
    fn test_align_1_000_000_times() {
        for _ in 0..1_000_000 {
            test_align();
        }
    }
}
