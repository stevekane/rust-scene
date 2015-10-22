use cgmath::{Vector2, Vector3, Matrix4};

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>, 
    pub normal: Vector3<f32>, 
    pub tex_coord: Vector2<f32>, 
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
}
