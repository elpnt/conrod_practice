extern crate glium;
extern crate winit;
extern crate conrod;

use glium::glutin;
use winit::{ControlFlow, Event, WindowEvent};

#[allow(unused_variables)]
fn main() {
    // Build the window
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello world")
        .with_dimensions(400, 300);
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    
    events_loop.run_forever(|event| {
        match event {
            Event::WindowEvent {event: WindowEvent::Closed, ..} => {
                println!("The window was closed ; stopping");
                ControlFlow::Break
            },
            _ => ControlFlow::Continue,
        }
    });
}
