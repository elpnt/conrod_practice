extern crate conrod;

#[allow(unused_variables)]
fn main() {
    // use conrod;
    use conrod::backend::glium::glium;
    // use conrod::backend::glium::glium::Surface;
    use conrod::backend::glium::glium::glutin::{ControlFlow, Event, WindowEvent};

    // Build the window
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello world")
        .with_dimensions(400, 300);
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    events_loop.run_forever(|event| {
        match event {
            Event::WindowEvent {event: WindowEvent::Closed, ..} => ControlFlow::Break,
            _ => ControlFlow::Continue,
        }
    });
    
}
