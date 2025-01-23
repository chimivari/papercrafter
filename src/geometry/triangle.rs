use crate::math::vec3f::Vec3F;


pub struct Triangle {
    vertices: [i32; 3],
    err: [f64; 4],
    deleted: i32,
    dirty: i32,
    attr: i32,
    n: Vec3F,
    uvs: Vec3F,
    material: i32,
}