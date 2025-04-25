use std::{f64::consts::PI, fmt::{Debug, Display}};

use nalgebra::{Matrix3, Vector3 as Vertex};

use super::face::Face;


pub struct Neighbour {
    pub child: usize,
    p1: Vertex<f64>,
    p_futherest: usize,
    c_futherest: usize,
    rot: Matrix3<f64>,
    pub collide: bool,
    pub angle: f64,
}

impl Neighbour {

    pub const EPSILON: f64 = 1e-8;

    pub fn new(parent: &Face, child: &Face, p_futherest: usize, c_futherest: usize) -> Self {
        let mut s = Self {
            child: child.id,
            p1: parent[(p_futherest + 1) % 3],
            p_futherest,
            c_futherest,
            rot: Matrix3::zeros(),
            collide: false,
            angle: 0.,
        };
        s.reload(&parent, &child);
        s
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

    /// Compute the [`rotation matrix`]
    /// 
    /// __Call this method during Mesh::flatten()__
    pub fn reload(&mut self, parent: &Face, child: &Face) {
        let p1 = (self.p_futherest + 1) % 3;
        let p2 = (self.p_futherest + 2) % 3;
        /* Compute u and p */
        let u = parent[p2] - parent[p1];
        let p = parent[p1];
        /* Compute rotation matrix */
        let mut p_axis = parent[self.p_futherest] - Self::project_on(&(parent[self.p_futherest] - p), &p, &u);
        assert!(p_axis.norm() > Self::EPSILON, "\x1b[41mFace too actue : {}\x1b[47m", parent);
        p_axis = p_axis.normalize();
        let wish = -p_axis;
        let mut c_axis = child[self.c_futherest] - Self::project_on(&(child[self.c_futherest] - p), &p, &u);
        assert!(c_axis.norm() > Self::EPSILON, "\x1b[31mFace too actue : {}\x1b[37m", child);
        c_axis = c_axis.normalize();
        let mut angle = Vertex::angle(&wish, &c_axis);
        let mut r = Self::compute_rot_mat(angle, &u);
        c_axis = r * c_axis;
        if (1. - Vertex::dot(&c_axis, &wish)).abs() > Self::EPSILON {
            r = r.transpose();
            angle = -angle;
        }
        self.angle = angle;
        self.rot = r;
    }

    /// Align [`face`] from here
    pub fn align_from_here(&self, face: &mut Face) {
        // Center face around (0, 0, 0)
        *face -= self.p1;
        // Rotate face
        face.rotate(self.rot);
        // Cancel centering
        *face += self.p1;
    }
}


impl Display for Neighbour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "child : {}", self.child)
    }
}

impl Debug for Neighbour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self::Display::fmt(&self, f)
    }
}
