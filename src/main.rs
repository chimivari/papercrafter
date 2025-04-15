use nalgebra::Vector3 as Vertex;

use mesh::Mesh;
use triangle::Triangle;

mod mesh;
mod triangle;

fn main() {
    let vertices = vec![
        Vertex::new(0., 0., 0.),
        Vertex::new(0., 0., 1.),
        Vertex::new(1., 0., 0.),
        Vertex::new(0., 1., 0.),
    ];
    let triangles = vec![
        Triangle(0, 1, 2),
        Triangle(0, 1, 3),
    ];
    let mut _mesh = Mesh::new(&triangles, &vertices);
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_init_unfolding() {
        let vertices = vec![
            Vertex::new(0., 0., 0.),
            Vertex::new(0., 0., 1.),
            Vertex::new(1., 0., 0.),
            Vertex::new(0., 1., 0.),
        ];
        let triangles = vec![
            Triangle(0, 1, 2),
            Triangle(0, 1, 3),
        ];
        let mut mesh = Mesh::new(&triangles, &vertices);
        mesh.init_unfolding(0);
        assert!(mesh.faces[0].is_flat());
        mesh.init_unfolding(1);
        assert!(mesh.faces[1].is_flat());
    }
}
