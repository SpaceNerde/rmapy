use wgpu::{Backend, Backends, RequestAdapterOptions};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::raw_window_handle::HasRawWindowHandle;


#[tokio::main]
async fn main() -> Result<(), impl std::error::Error> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Ensure that the window implements HasRawWindowHandle
    let raw_handle = window.raw_window_handle().unwrap();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance.request_adapter(&RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }).await.unwrap();
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        },
        None,
    ).await.unwrap();

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event,
                window_id
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                _ => (),
            },
            Event::AboutToWait => {
                window.request_redraw();
            }

            _ => (),
        }
    })
}