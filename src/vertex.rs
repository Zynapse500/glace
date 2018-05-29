

gfx_defines!(
    vertex Vertex {
        position: [f32; 3] = "position",
        color: [f32; 4] = "color",
    }
);


impl Vertex {
    pub fn new(position: [f32; 3]) -> Vertex {
        Vertex {
            position,
            color: [1.0, 1.0, 1.0, 1.0]
        }
    }


    pub fn with_color(self, color: [f32; 4]) -> Vertex {
        Vertex {
            color,
            ..self
        }
    }
}
