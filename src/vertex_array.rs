use glium::{
    Display,
    VertexBuffer,
    IndexBuffer,
    vertex::VertexBufferSlice,
    index::{
        PrimitiveType,
        IndexBufferSlice,
    },
};

use vertex::Vertex;

pub struct VertexArray {
    display: Display,

    arrays: Vec<(PrimitiveType, Array)>,
}


struct Array {
    vertex_buffer: VertexBuffer<Vertex>,
    vertices: Vec<Vertex>,

    index_buffer: IndexBuffer<u32>,
    indices: Vec<u32>,
}


impl VertexArray {
    pub fn new(display: Display) -> VertexArray {
        VertexArray {
            display,

            arrays: Vec::new(),
        }
    }


    pub fn clear_vertices(&mut self, primitive: PrimitiveType) {
        if let Some(index) = self.get_array_index(primitive) {
            let array = &mut self.arrays[index].1;
            array.vertices.clear();
            array.indices.clear();
        }
    }


    pub fn clear_all_vertices(&mut self) {
        for a in self.arrays.iter_mut() {
            let array = &mut a.1;
            array.vertices.clear();
            array.indices.clear();
        }
    }


    pub fn append_vertices(&mut self, vertices: &[Vertex], indices: &[u32], primitive: PrimitiveType) {
        if let Some(index) = self.get_array_index(primitive) {
            let array = &mut self.arrays[index].1;

            let start = array.vertices.len() as u32;
            array.indices.extend(indices.iter().map(|v|{*v + start}));
            array.vertices.extend(vertices);
        } else {
            let array = Array {
                vertex_buffer: VertexBuffer::empty_dynamic(&self.display, 0).unwrap(),
                vertices: vertices.to_vec(),
                index_buffer: IndexBuffer::empty_dynamic(&self.display, primitive, 0).unwrap(),
                indices: indices.to_vec(),
            };

            self.arrays.push((primitive, array));
        }
    }


    pub fn get_vertices<'a>(&'a mut self) -> Vec<(VertexBufferSlice<'a, Vertex>, IndexBufferSlice<'a, u32>)> {
        let mut slices = Vec::new();
        for a in self.arrays.iter_mut() {
            let array = &mut a.1;

            if array.vertices.len() > 0 && array.indices.len() > 0 {
                if array.vertex_buffer.len() < array.vertices.len() {
                    array.vertex_buffer = VertexBuffer::new(&self.display, &array.vertices).unwrap();
                } else {
                    array.vertex_buffer.slice(..array.vertices.len()).unwrap().write(&array.vertices);
                }

                if array.index_buffer.len() < array.indices.len() {
                    array.index_buffer = IndexBuffer::new(&self.display, a.0, &array.indices).unwrap();
                } else {
                    array.index_buffer.slice(..array.indices.len()).unwrap().write(&array.indices);
                }

                slices.push((
                    array.vertex_buffer.slice(..array.vertices.len()).unwrap(),
                    array.index_buffer.slice(..array.indices.len()).unwrap()
                ));
            }
        }

        slices
    }

    fn get_array_index<'a>(&self, primitive: PrimitiveType) -> Option<usize> {
        for (i, &(p, _)) in self.arrays.iter().enumerate() {
            if p == primitive {
                return Some(i);
            }
        }

        None
    }
}
