
use gfx::{
    traits::FactoryExt
};

use pipeline::*;

use draw::{
    Draw,
    DrawCommand,
};

use trap::Matrix4;

use color::Color;

use projection::{
    Projection,
    View
};

use vertex::Vertex;
use vertex_array::VertexArray;

pub struct Frame {
    factory: ::Factory,
    encoder: ::Encoder,
    pipeline: ::PipelineState,

    color_view: ::RenderTargetView,
    depth_stencil_view: ::DepthStencilView,

    projection_matrix: Matrix4,
    view_matrix: Matrix4,

    vertex_array: VertexArray
}


impl Frame {
    pub fn new(
        mut factory: ::Factory,
        pipeline: ::PipelineState,
        color_view: ::RenderTargetView,
        depth_stencil_view: ::DepthStencilView,
        vertex_array: VertexArray
    ) -> Frame {
        let encoder = ::Encoder::from(factory.create_command_buffer());

        Frame {
            factory,
            encoder,
            pipeline,

            color_view,
            depth_stencil_view,

            projection_matrix: Matrix4::new(),
            view_matrix: Matrix4::new(),

            vertex_array
        }
    }


    pub fn consume(mut self) -> (::Encoder, VertexArray) {
        self.flush();

        (self.encoder, self.vertex_array)
    }


    pub fn clear(&mut self, color: Color) {
        self.encoder.clear(
            &self.color_view,
            color.into()
        );

        self.clear_depth();
    }

    pub fn clear_depth(&mut self) {
        self.encoder.clear_depth(
            &self.depth_stencil_view,
            1.0
        );
    }


    /// Render something
    pub fn draw(&mut self, object: &Draw) {
        self.execute_draw_command(object.draw());
    }



    /// Set the projection mode
    pub fn set_projection(&mut self, projection: Projection) {
        self.flush();

        match projection {
            Projection::Perspective { fov, aspect, near, far } => {
                self.projection_matrix = Matrix4::perspective(fov, aspect, near, far);
            }

            Projection::Orthographic { left, right, top, bottom, near, far } => {
                self.projection_matrix = Matrix4::orthographic(left, right, top, bottom, near, far);
            }
        }
    }


    /// Set the view mode
    pub fn set_view(&mut self, view: View) {
        self.flush();

        match view {
            View::LookAt { eye, target, up } => {
                self.view_matrix = Matrix4::look_at(eye, target, up);
            }

            View::None => {
                self.view_matrix = Matrix4::new();
            }
        }
    }
}


impl Frame {
    fn flush(&mut self) {
        self.draw_vertices();

        self.vertex_array.clear_vertices();
    }

    fn execute_draw_command(&mut self, command: DrawCommand) {
        match command {
            DrawCommand::IndexedVertices { vertices, indices } => {
                self.vertex_array.append_vertices(vertices.as_slice(), indices.as_slice());
            },

            DrawCommand::List(commands) => {
                for command in commands.into_iter() {
                    self.execute_draw_command(command);
                }
            }
        }
    }

    fn draw_vertices(&mut self) {
        let vertices = self.vertex_array.get_vertices();
        let indices = self.vertex_array.get_indices();

        if vertices.len() > 0 && indices.len() > 0 {
            let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(
                vertices,
                indices
            );

            let transform_buffer = self.factory.create_constant_buffer(1);

            let data = pipe::Data {
                vbuf: vertex_buffer,
                transform: transform_buffer,
                out_color: self.color_view.clone(),
                out_depth_stencil: self.depth_stencil_view.clone()
            };

            self.encoder.update_buffer(
                &data.transform,
                &[Transform {
                    projection: self.projection_matrix.into(),
                    view: self.view_matrix.into()
                }],
                0
            );

            self.encoder.draw(
                &slice,
                &self.pipeline,
                &data
            );
        }
    }
}


fn mat_to_arr(matrix: Matrix4) -> [[f32; 4]; 4] {
    matrix.into()
}