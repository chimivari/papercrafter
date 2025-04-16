use nalgebra::{Vector2, Vector3 as Vertex};

use mesh::Mesh;
use triangle::Triangle;

mod mesh;
mod triangle;

fn main() {
    // let vertices = vec![
    //     Vertex::new(0., 0., 0.),
    //     Vertex::new(0., 0., 1.),
    //     Vertex::new(1., 0., 0.),
    //     Vertex::new(0., 1., 0.),
    // ];
    // let triangles = vec![
    //     Triangle(0, 1, 2),
    //     Triangle(0, 1, 3),
    // ];
    // let mut mesh = Mesh::new(&triangles, &vertices);
    // mesh.neighbours[0].before_unfold(&mesh.faces[0], &mesh.faces[1]);
}


#[cfg(test)]
mod test {
    use rand::Rng;

    use super::*;

    fn random_vertex(rng: &mut rand::rngs::ThreadRng) -> Vertex<f64> {
        Vertex::new(rng.random_range(-10.0..10.0), rng.random_range(-10.0..10.0), rng.random_range(-10.0..10.0))
    }

    fn random_point(rng: &mut rand::rngs::ThreadRng) -> Vertex<f64> {
        Vertex::new(rng.random_range(-10.0..10.0), 0., rng.random_range(-10.0..10.0))
    }

    #[test]
    fn test_unfolding_from_here() {
        let mut rng = rand::rng();
        let vertices = vec![
            random_point(&mut rng),
            random_point(&mut rng),
            random_point(&mut rng),
            random_point(&mut rng),
        ];
        let triangles = vec![
            Triangle(0, 1, 2),
            Triangle(0, 1, 3),
        ];
        let mut mesh = Mesh::new(&triangles, &vertices);
        mesh.init_unfolding(0);
        assert!(mesh.faces[0].is_flat());
        mesh.neighbours[mesh.faces[0].neighbourhoud[0]].before_unfold(&mesh.faces[0], &mesh.faces[1]);
        mesh.neighbours[mesh.faces[0].neighbourhoud[0]].unfold_from_here(mesh.faces[0][mesh.neighbours[mesh.faces[0].neighbourhoud[0]].p1], &mut mesh.faces[1]);
        assert!(mesh.faces[0].is_flat());
        assert!(mesh.faces[1].is_flat());
        assert!(!mesh.faces[0].collide(&mesh.faces[1]), "collide : {mesh}");
    }

    #[test]
    fn test_unfolding_from_here_1_000_000() {
        for _ in 0..1_000_000 {
            test_unfolding_from_here();
        }
    }
}
