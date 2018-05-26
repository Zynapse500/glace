
use trap::{
    Matrix4
};

use vertex::Vertex;

pub enum Triangles {
    IndexedList {
        vertices: Vec<Vertex>,
        indices: Vec<u32>
    }
}


pub trait Draw {
    fn triangulate(&self) -> Triangles;

    fn transformation(&self) -> Matrix4 {
        Matrix4::new()
    }
}
