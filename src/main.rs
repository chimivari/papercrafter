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
        let mesh_ref = Mesh::new(&triangles, &vertices);
        let mut mesh = Mesh::new(&triangles, &vertices);
        let f1 = 0;
        let f2 = 1;
        
        let f1_n1 = 0;
        mesh.align(f1, f2, mesh.faces[f1].neighbourhoud[f1_n1]);
        assert!(mesh.faces[f1].is_aligned_to(&mesh.faces[f2]), "{mesh_ref}\n{mesh}");
    }

    #[test]
    fn test_align_one() {
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
        let ref_mesh = Mesh::new(&triangles, &vertices);
        let f1 = 0;
        let f2 = 1;
        
        let f1_n1 = 0;
        mesh.align(f1, f2, mesh.faces[f1].neighbourhoud[f1_n1]);
        if !mesh.faces[f1].is_aligned_to(&mesh.faces[f2]) {
            println!("{ref_mesh}");
        }
        assert!(mesh.faces[f1].is_aligned_to(&mesh.faces[f2]));
    }

    #[test]
    fn test_align_1_000_000() {
        for _ in 0..1_000_000 {
            test_align_one();
        }
    }
}
