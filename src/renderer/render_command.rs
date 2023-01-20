use futures_lite::future;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::{Backends, Instance, PowerPreference, RequestAdapterOptions};

struct RenderCommand;

impl RenderCommand {
    fn new<T>(window: &T) -> Self
    where
        T: HasRawWindowHandle + HasRawDisplayHandle,
    {
        // Initialize wgpu.
        let wgpu_instance = Instance::new(Backends::all());

        // Create the wgpu platform surface from the window.
        let wgpu_surface = unsafe { wgpu_instance.create_surface(window) };

        // Retrieve the handle to the GPU based on the created surface.
        // First, prepare the adapter options.
        let wgpu_request_adapter_options = RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&wgpu_surface),
            force_fallback_adapter: false,
        };

        // Now, retrieve the adapter according to the prepared options.
        let wgpu_adapter =
            future::block_on(wgpu_instance.request_adapter(&wgpu_request_adapter_options));

        Self
    }
}
