extern crate graphics_3d;

use graphics_3d::{
    EventsLoop,
    WindowBuilder,
    ContextBuilder,
    Screen,

    Event,
    WindowEvent,

    Color,

    DrawCommand,
    Draw,
    Vertex
};

fn main() {
    let mut events_loop = EventsLoop::new();

    let mut screen = {
        let window = WindowBuilder::new()
            .with_dimensions(800, 600)
            .with_title("Window");

        let context = ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(8);

        Screen::new(window, context, &events_loop)
    };


    let mut running = true;

    let mut time: f32 = 0.0;

    while running {
        events_loop.poll_events(|e|{
            match e {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::Closed => {
                            running = false;
                        },

                        _ => ()
                    }
                },

                _ => ()
            }
        });

        time += 1.0 / 60.0;


        screen.render(|mut frame| {
            frame.clear(Color::new(0.01, 0.01, 0.01, 1.0));


            frame.draw(&HelloTriangle {
                x: time.cos() * 0.5,
                y: time.sin() * 0.5
            });
        });
    }
}


struct HelloTriangle {
    x: f32,
    y: f32
}



impl Draw for HelloTriangle {
    fn draw(&self) -> DrawCommand {
        DrawCommand::IndexedVertices {
            vertices: vec![
                Vertex { position: [self.x + 0.0, self.y + 0.5, 0.0], color: [1.0, 0.0, 0.0, 1.0] },
                Vertex { position: [self.x + 0.5, self.y - 0.5, 0.0], color: [0.0, 1.0, 0.0, 1.0] },
                Vertex { position: [self.x - 0.5, self.y - 0.5, 0.0], color: [0.0, 0.0, 1.0, 1.0] }
            ],

            indices: vec![0, 1, 2],
        }
    }
}
