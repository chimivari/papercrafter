use std::{ffi::{c_char, CString}, fmt::Display};

struct Vertex {
    x: f64,
    y: f64,
    z: f64,
}
impl Vertex {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }
}
impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}
impl Triangle {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self {a, b, c}
    }
}
impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.a, self.b, self.c)
    }
}

#[link(name = "MeshSimplifier", kind = "dylib")]
extern {
    fn simplify(file_path: *const c_char, export_path: *const c_char, reduce_fraction: f32, agressiveness: f32) -> i32;

    fn get_vertices(count: *mut usize) -> *mut f64;
    fn free_vertices(ptr: *mut f64);

    fn get_triangles(count: *mut usize) -> *mut usize;
    fn free_triangles(ptr: *mut usize);
}

fn main() {
    unsafe  {
        let input_path = CString::new(r"C:\Users\Andeol\Videos\rust_projects\papercrafter\models\shpere_100_faces.obj")
            .unwrap();
        let export_path = CString::new(r"exported_path.obj").unwrap();
        simplify(
            input_path.as_ptr(),
            export_path.as_ptr(), 
            0.7, 
            7.
        );

        let mut size = 0;
        let ptr = get_vertices(&mut size);
        if ptr.is_null() {
            eprintln!("Returned ptr is null");
        }
        let vertices_slice = std::slice::from_raw_parts(ptr, size);
        let mut vertices = Vec::new();
        let mut i = 0;
        while i < size {
            let vertex = Vertex::new(
                vertices_slice[i], 
                vertices_slice[i + 1], 
                vertices_slice[i + 2],
            );
            vertices.push(vertex);
            i += 3;
        }
        free_vertices(ptr);

        let mut size = 0;
        let ptr = get_triangles(&mut size);
        if ptr.is_null() {
            eprintln!("Returned ptr is null");
        }
        let triangles_slice = std::slice::from_raw_parts(ptr, size);
        let mut triangles = Vec::new();
        let mut i = 0;
        while i < size {
            let triangle = Triangle::new(
                triangles_slice[i], 
                triangles_slice[i + 1], 
                triangles_slice[i + 2],
            );
            triangles.push(triangle);
            i += 3;
        }
        free_triangles(ptr);
    }
}