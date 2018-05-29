
use vertex::Vertex;

pub struct VertexArray {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}


impl VertexArray {
    pub fn new() -> VertexArray {
        VertexArray {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }


    pub fn clear_vertices(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }


    pub fn append_vertices(&mut self, vertices: &[Vertex], indices: &[u32]) {
        let offset = self.vertices.len() as u32;
        self.indices.extend(indices.iter().map(|i|{*i + offset}));
        self.vertices.extend(vertices);
    }


    pub fn get_vertices<'a>(&'a self) -> &'a [Vertex] {
        self.vertices.as_slice()
    }

    pub fn get_indices<'a>(&'a self) -> &'a [u32] {
        self.indices.as_slice()
    }
}
