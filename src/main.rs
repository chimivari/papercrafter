use std::ffi::{c_char, CString};

#[link(name = "MeshSimplifier", kind = "dylib")]
extern {
    fn simplify(file_path: *const c_char, export_path: *const c_char, reduce_fraction: f32, agressiveness: f32) -> i32;
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
    }
}