use futures_lite::future;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::{
    Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference, PresentMode, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};

use crate::error;

struct RenderCommand {
    wgpu_surface: Surface,
    wgpu_device: Device,
    wgpu_queue: Queue,
    wgpu_surface_configuration: SurfaceConfiguration,
}

impl RenderCommand {
    fn new<T>(window: &T, window_width: u32, window_height: u32) -> error::Result<Self>
    where
        T: HasRawWindowHandle + HasRawDisplayHandle,
    {
        // Initialize wgpu.
        let wgpu_instance = Instance::default();

        // Create the wgpu platform surface from the window.
        let wgpu_surface = unsafe { wgpu_instance.create_surface(window) }
            .map_err(error::Error::WgpuCreateSurface)?;

        // Retrieve the handle to the GPU based on the created surface.
        // First, prepare the adapter options.
        let wgpu_request_adapter_options = RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&wgpu_surface),
        };

        // Now, retrieve the wgpu adapter according to the prepared options.
        let wgpu_adapter =
            future::block_on(wgpu_instance.request_adapter(&wgpu_request_adapter_options))
                .ok_or(error::Error::WgpuRequestAdapter)?;

        // Create a wgpu device and a wgpu queue using the adapter.
        let wgpu_device_descriptor = DeviceDescriptor {
            features: Features::empty(),
            limits: Limits::default(),
            label: None,
        };
        let (wgpu_device, wgpu_queue) =
            future::block_on(wgpu_adapter.request_device(&wgpu_device_descriptor, None))
                .map_err(error::Error::WgpuRequestDevice)?;

        // Retrieve the swapchain's capabilities.
        let wgpu_swapchain_capabilities = wgpu_surface.get_capabilities(&wgpu_adapter);
        let wgpu_swapchain_format = wgpu_swapchain_capabilities.formats[0];

        // Configure the surface for presentation.
        let wgpu_surface_configuration = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: wgpu_swapchain_format,
            width: window_width,
            height: window_height,
            present_mode: PresentMode::Fifo,
            alpha_mode: wgpu_swapchain_capabilities.alpha_modes[0],
            view_formats: Vec::new(),
        };
        wgpu_surface.configure(&wgpu_device, &wgpu_surface_configuration);

        Ok(Self {
            wgpu_surface,
            wgpu_device,
            wgpu_queue,
            wgpu_surface_configuration,
        })
    }
}
