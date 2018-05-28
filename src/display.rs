

use glutin::{
    WindowBuilder,
    ContextBuilder,
    EventsLoop,

    GlWindow,

    GlContext
};

use gfx_window_glutin;


use gfx::{
    format::{
        Srgba8
    },

    handle::{
        RenderTargetView,
        DepthStencilView
    },

    Encoder,
};



pub struct Display {
    window: GlWindow,

    device: ::Device,
    factory: ::Factory,
    color_view: RenderTargetView<::Resources, ::ColorFormat>,
    depth_stencil_view: DepthStencilView<::Resources, ::DepthFormat>
}


impl Display {
    pub fn new(
        window: WindowBuilder,
        context: ContextBuilder,
        events_loop: &EventsLoop
    ) -> Display {
        let (
            gl_window,
            device,
            factory,
            color_view,
            depth_stencil_view
        ) = gfx_window_glutin::init::<::ColorFormat, ::DepthFormat>(
            window,
            context,
            events_loop
        );



        Display {
            window: gl_window,

            device,
            factory,
            color_view,
            depth_stencil_view
        }
    }


    pub fn clear(&mut self) {
        let mut command_buffer = self.factory.create_command_buffer();
        let mut encoder = Encoder::from(command_buffer);

        encoder.clear(&self.color_view, [0.2, 0.2, 0.2, 1.0]);
        encoder.flush(&mut self.device);

        self.window.swap_buffers().unwrap();
    }
}

