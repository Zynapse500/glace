use glutin::{
    WindowBuilder,
    ContextBuilder,
    EventsLoop,
    GlWindow,
    GlContext,
    CursorState,
};

use gfx_window_glutin;

use gfx::{
    Device,
    format::Srgba8,
    handle::{
        RenderTargetView,
        DepthStencilView,
    },
    ShaderSet,
    VertexShader,
    PixelShader,
    shade::core::Stage,
    Primitive,
    state::{
        Rasterizer,
        FrontFace,
        CullFace,
        RasterMethod,
        Offset,
        MultiSample,
    },
    PipelineState,
    traits::FactoryExt,
    Factory,
    Encoder,
};


use frame::Frame;

use vertex::Vertex;
use pipeline::*;
use vertex_array::VertexArray;

pub struct Display {
    window: GlWindow,

    device: ::Device,
    factory: ::Factory,
    color_view: ::RenderTargetView,
    depth_stencil_view: ::DepthStencilView,

    pipeline: ::PipelineState,

    vertex_arrays: Vec<VertexArray>
}


impl Display {
    pub fn new(
        window: WindowBuilder,
        context: ContextBuilder,
        events_loop: &EventsLoop,
    ) -> Display {
        let (
            gl_window,
            device,
            mut factory,
            color_view,
            depth_stencil_view
        ) = gfx_window_glutin::init::<::ColorFormat, ::DepthFormat>(
            window,
            context,
            events_loop,
        );


        let vertex_shader = VertexShader::new(factory.create_shader(
            Stage::Vertex, include_bytes!("shaders/shader.vert"),
        ).unwrap());

        let pixel_shader = PixelShader::new(factory.create_shader(
            Stage::Pixel, include_bytes!("shaders/shader.frag"),
        ).unwrap());

        let shader_set = ShaderSet::Simple(
            vertex_shader,
            pixel_shader,
        );

        let pipeline = factory.create_pipeline_state(
            &shader_set,
            Primitive::TriangleList,
            Rasterizer {
                front_face: FrontFace::Clockwise,
                cull_face: CullFace::Nothing,
                method: RasterMethod::Fill,
                offset: None,
                samples: Some(MultiSample),
            },
            pipe::new(),
        ).unwrap();

        Display {
            window: gl_window,

            device,
            factory,
            color_view,
            depth_stencil_view,

            pipeline,

            vertex_arrays: Vec::new(),
        }
    }


    pub fn render(&mut self) -> Frame {
        gfx_window_glutin::update_views(
            &self.window,
            &mut self.color_view,
            &mut self.depth_stencil_view,
        );

        Frame::new(
            self.factory.clone(),
            self.pipeline.clone(),
            self.color_view.clone(),
            self.depth_stencil_view.clone(),
            self.get_vertex_array()
        )
    }


    pub fn submit(&mut self, mut frame: Frame) {
        let (mut encoder, vertex_array) = frame.consume();
        self.vertex_arrays.push(vertex_array);

        encoder.flush(&mut self.device);

        self.window.swap_buffers().unwrap();
        self.device.cleanup();
    }


    pub fn get_size(&self) -> (u32, u32) {
        self.window.get_inner_size().unwrap()
    }


    pub fn set_cursor_state(&mut self, state: CursorState) {
        self.window.set_cursor_state(state).unwrap()
    }
}


impl Display {
    fn get_vertex_array(&mut self) -> VertexArray {
        if let Some(array) = self.vertex_arrays.pop() {
            array
        } else {
            VertexArray::new()
        }
    }
}
