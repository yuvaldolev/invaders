use futures_lite::future;
use wgpu::{
    Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference, PresentMode, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};

use crate::error;
use crate::window::Window;

pub struct RenderCommand {
    surface: Surface,
    device: Device,
    queue: Queue,
    surface_configuration: SurfaceConfiguration,
}

impl RenderCommand {
    pub fn new(window: &Window) -> error::Result<Self> {
        // Initialize wgpu.
        let instance = Instance::default();

        // Create the wgpu platform surface from the window.
        let surface = unsafe { instance.create_surface(window.get_glfw_window()) }
            .map_err(error::Error::WgpuCreateSurface)?;

        // Retrieve the handle to the GPU based on the created surface.
        // First, prepare the adapter options.
        let request_adapter_options = RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        };

        // Now, retrieve the wgpu adapter according to the prepared options.
        let adapter = future::block_on(instance.request_adapter(&request_adapter_options))
            .ok_or(error::Error::WgpuRequestAdapter)?;

        // Create a wgpu device and a wgpu queue using the adapter.
        let device_descriptor = DeviceDescriptor {
            features: Features::empty(),
            limits: Limits::default(),
            label: None,
        };
        let (device, queue) = future::block_on(adapter.request_device(&device_descriptor, None))
            .map_err(error::Error::WgpuRequestDevice)?;

        // Retrieve the swapchain's capabilities.
        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        // Configure the surface for presentation.
        let surface_configuration = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: window.get_width(),
            height: window.get_height(),
            present_mode: PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: Vec::new(),
        };
        surface.configure(&device, &surface_configuration);

        Ok(Self {
            surface,
            device,
            queue,
            surface_configuration,
        })
    }
}
