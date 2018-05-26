

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4]
}






implement_vertex!(Vertex, position, color);