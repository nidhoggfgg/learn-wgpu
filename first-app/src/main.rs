use first_app::FristAppHandler;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = FristAppHandler::new();
    event_loop.run_app(&mut app).unwrap_or(())
}
