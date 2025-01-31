use std::ffi::{c_char, CString};

#[link(name = "MeshSimplifier", kind = "dylib")]
extern {
    fn simplify(file_path: *const c_char, export_path: *const c_char, reduce_fraction: f32, agressiveness: f32) -> i32;
    fn get_vertices(count: *mut usize) -> *mut f64;
    fn free_vertices(ptr: *mut f64);
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

        let slice = std::slice::from_raw_parts(ptr, size);
        println!("{:?}", slice);
        free_vertices(ptr);
    }
}