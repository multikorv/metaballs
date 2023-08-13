mod metaball;

use metaball::drawer::Drawer;
use softbuffer::{
    Context,
    Surface,
};

use winit::{
    window::{
        WindowBuilder,
        Window
    },
    event_loop::{
        EventLoop,
        ControlFlow
    },
    event::{
        Event,
        WindowEvent
    }
};

fn main() {
    let event_loop: EventLoop<()> = EventLoop::new();

    let window: Window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let context: Context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let surface: Surface = unsafe { softbuffer::Surface::new(&context, &window).unwrap() };
    let mut drawer = Drawer::new(surface);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                drawer.resize(width, height);
                drawer.write();
                drawer.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit
            },
            _ => ()
        }
    });
}