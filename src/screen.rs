use glium;

use frame::Frame;


use glium::{
    Display,
    Program,

    glutin::{
        EventsLoop,
        WindowBuilder,
        ContextBuilder,
        CursorState
    }
};

use vertex_array::VertexArray;

use std::str::from_utf8;

pub struct Screen {
    display: Display,

    vertex_array: VertexArray,
    program: Program
}


impl Screen {
    pub fn new(
        window: WindowBuilder,
        context: ContextBuilder,
        events_loop: &EventsLoop
    ) -> Screen {
        let display = glium::Display::new(
            window,
            context,
            events_loop).unwrap();

        let vertex_array = VertexArray::new(display.clone());


        let program = Program::from_source(
            &display,
            from_utf8(include_bytes!("shaders/shader.vert")).unwrap(),
            from_utf8(include_bytes!("shaders/shader.frag")).unwrap(),
            None
        ).unwrap();

        Screen {
            display,
            vertex_array,

            program
        }
    }


    pub fn render<F>(&mut self, mut f: F)
        where F: FnMut(Frame) {
        let frame = Frame::new(
            self.display.draw(),
            &mut self.vertex_array,
            &self.program
        );

        f(frame);
    }



    /// Return the size of the window
    pub fn get_size(&self) -> (u32, u32) {
        self.display.gl_window().get_inner_size().unwrap()
    }


    /// Sets the state of the cursor
    pub fn set_cursor(&mut self, state: CursorState) {
        self.display.gl_window().window().set_cursor_state(state).unwrap();
    }
}
