use std::sync::Arc;

use parking_lot::Mutex;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window,
};

mod utils;

pub struct FirstApp {
    #[allow(unused)]
    window: Window,
}

impl FirstApp {
    pub async fn new(window: Window) -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            const CANVAS_ID: &str = "webgpu_app_canvas";
            const CONTAINER_ID: &str = "webgpu_app_container";

            use wasm_bindgen::JsValue;
            use winit::platform::web::WindowExtWebSys;
            let canvas = window.canvas().unwrap();

            web_sys::window().and_then(|win| win.document()).map(|doc| {
                canvas.set_attribute("id", CANVAS_ID).unwrap_or(());
                if let Some(container) = doc.get_element_by_id(CONTAINER_ID) {
                    container
                        .append_child(&canvas)
                        .expect(&format!("can't add canvas to {}", CONTAINER_ID));
                } else {
                    (|| -> Result<(), JsValue> {
                        let container = doc.create_element("div")?;
                        container.set_attribute("id", CONTAINER_ID)?;
                        container.append_child(&canvas)?;
                        Ok(())
                    })()
                    .expect(&format!(
                        "can't create {} and add canvas to it",
                        CONTAINER_ID
                    ));
                }
            });

            canvas.set_tab_index(0);
        }
        Self { window }
    }
}

pub struct FristAppHandler {
    #[allow(unused)]
    app: Arc<Mutex<Option<FirstApp>>>,
}

impl ApplicationHandler for FristAppHandler {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.app.lock().is_some() {
            return;
        }

        let attr = Window::default_attributes().with_title("webgpu_app_window");
        let window = event_loop.create_window(attr).unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            let app = self.app.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let wgpu_app = FirstApp::new(window).await;
                let mut app = app.lock();
                *app = Some(wgpu_app);
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let app = pollster::block_on(FirstApp::new(window));
            self.app.lock().replace(app);
        }
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if event == WindowEvent::CloseRequested {
            event_loop.exit();
        }
    }
}

impl FristAppHandler {
    pub fn new() -> Self {
        Self {
            app: Arc::new(Mutex::new(None)),
        }
    }
}

impl Default for FristAppHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    utils::init_logger();

    let events_loop = EventLoop::new().unwrap();
    let mut app = FristAppHandler::new();
    events_loop.run_app(&mut app).unwrap_or(());
}
