
use trap::{
    Matrix4
};

use vertex::Vertex;

pub enum DrawCommand {
    IndexedVertices {
        vertices: Vec<Vertex>,
        indices: Vec<u32>
    },

    List(Vec<DrawCommand>)
}


pub trait Draw {
    fn draw(&self) -> DrawCommand;
}
