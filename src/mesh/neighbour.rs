use std::f64::consts::PI;

use nalgebra::{Matrix3, Vector3 as Vertex};

use super::face::Face;


pub struct Neighbour {
    pub child: usize,
    pub p1: usize,
    pub p2: usize,
    pub p_futherest: usize,
    pub c_futherest: usize,
    rot: Matrix3<f64>,
    pub collide: bool,
}

impl Neighbour {

    pub const EPSILON: f64 = 1e-8;

    pub fn new(parent: &Face, child: &Face, p_futherest: usize, c_futherest: usize) -> Self {
        let p1 = (p_futherest + 1) % 3;
        let p2 = (p_futherest + 2) % 3;
        /* Compute u and p */
        let u = parent[p2] - parent[p1];
        let p = parent[p1];
        /* Compute rotation matrix */
        let p_axis =
            (parent[p_futherest] - Self::project_on(&(parent[p_futherest] - p), &p, &u))
            .normalize();
        let wish = -p_axis;
        let mut c_axis =
            (child[c_futherest] - Self::project_on(&(child[c_futherest] - p), &p, &u))
            .normalize();
        let angle = Vertex::angle(&wish, &c_axis);
        let mut r = Self::compute_rot_mat(angle, &u);
        c_axis = r * c_axis;
        if (1. - Vertex::dot(&c_axis, &wish)).abs() > Self::EPSILON {
            r = r.transpose();
        }
        Self {
            child: child.id,
            p1: (p_futherest + 1) % 3,
            p2: (p_futherest + 2) % 3,
            p_futherest,
            c_futherest,
            rot: r,
            collide: false,
        }
    }

    /// Project [`u`] on [`axis`] passing through [`p`]
    pub fn project_on(u: &Vertex<f64>, p: &Vertex<f64>, axis: &Vertex<f64>) -> Vertex<f64> {
        let norm2 = axis.norm_squared();
        let projection = (Vertex::dot(u, axis) * axis) / norm2 + p;
        projection
    }

    /// Compute the rotation matrix
    fn compute_rot_mat(theta: f64, u: &Vertex<f64>) -> Matrix3<f64> {
        let u = u.normalize();
        let (ux, uy, uz) = (u.x, u.y, u.z);
        let cos = theta.cos();
        let sin = theta.sin();
        let omcos = 1. - cos;
        Matrix3::new(
            ux*ux*omcos+cos, ux*uy*omcos-uz*sin, ux*uz*omcos+uy*sin,
            ux*uy*omcos+uz*sin, uy*uy*omcos+cos, uy*uz*omcos-ux*sin,
            ux*uz*omcos-uy*sin, uy*uz*omcos+ux*sin, uz*uz*omcos+cos
        )
    }

    // fn before_unfold(&mut self, parent: &Face, child: &Face) {
    //     if child.id != self.child {return}
    //     /* Compute u and p */
    //     let u = parent[self.p2] - parent[self.p1];
    //     let p = parent[self.p1];
    //     /* Compute rotation matrix */
    //     let p_axis =
    //         (parent[self.p_futherest] - Self::intersect(&p, &parent[self.p_futherest], &u))
    //         .normalize();
    //     let wish = -p_axis;
    //     let mut c_axis =
    //         (child[self.c_futherest] - Self::intersect(&p, &child[self.c_futherest], &u))
    //         .normalize();
    //     let angle = Vertex::angle(&wish, &c_axis);
    //     let mut r = Self::compute_rot_mat(angle, &u);
    //     c_axis = r * c_axis;
    //     // println!("{c_axis}");
    //     if (1. - Vertex::dot(&c_axis, &wish)).abs() > Self::EPSILON {
    //         r = r.transpose();
    //     }
    //     self.rot = r;
    // }

    /// Align [`face`] from here
    pub fn align_from_here(&self, p: Vertex<f64>, face: &mut Face) {
        // Center face around (0, 0, 0)
        *face -= p;
        // Rotate face
        face.rotate(self.rot);
        // Cancel centering
        *face += p;
    }
}