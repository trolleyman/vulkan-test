use std::sync::Arc;
use std::ptr;

use glutin::{WindowBuilder, Window, Event};

use vulkano;
use vulkano::format;
use vulkano::instance::{InstanceExtensions, Instance, PhysicalDevice, DeviceExtensions};
use vulkano::device::Device;
use vulkano::swapchain::{Swapchain, Surface, SurfaceTransform, CompositeAlpha};
use vulkano::command_buffer::{CommandBufferPool, Submission, PrimaryCommandBufferBuilder};
use vulkano::buffer::{self, Buffer};
use vulkano::memory::HostVisible;
use vulkano::image::{self, ImageView};
use vulkano::framebuffer::{Framebuffer, RenderPass};
use vulkano::pipeline;
use vulkano::descriptor_set::{PipelineLayout, EmptyPipelineDesc};

use shaders;

#[cfg(windows)]
fn get_surface(instance: &Arc<Instance>, w: &Window) -> Arc<Surface> {
	use glutin::os::windows::WindowExt;
	unsafe {
		let hwnd = w.get_hwnd();
		
		Surface::from_hwnd(instance, ptr::null() as *const (), hwnd).expect("failed to create surface")
	}
}

#[cfg(windows)]
fn get_platform_specific_extensions() -> InstanceExtensions {
	InstanceExtensions {
		khr_win32_surface: true,
		.. InstanceExtensions::none()
	}
}

pub struct Render {
	window: Window,
	submissions: Vec<Submission>,
}
impl Render {
	pub fn new() -> Result<Render, String> {
		let extensions = InstanceExtensions {
			khr_surface: true,
			//khr_display: true,
			//khr_display_swapchain: true,
			.. get_platform_specific_extensions()
		};
		let instance = try!(Instance::new(None, &extensions, &[])
			.map_err(|e| format!("Failed to create instance: {}", e)));
		
		for (i, physical) in PhysicalDevice::enumerate(&instance).enumerate() {
			println!("Device {}: {} (type: {:?})", i, physical.name(), physical.ty());
		}
		
		let physical = try!(PhysicalDevice::enumerate(&instance).next()
			.ok_or(format!("No devices found.")));
		
		println!("Using device: {} (type: {:?})", physical.name(), physical.ty());
		
		let window = try!(WindowBuilder::new()
			.with_title("Vulkan Test".into())
			.build()
			.map_err(|e| format!("Failed to create window: {}", e)));
		
		let surface = get_surface(&instance, &window);
		
		// Find command queue. Multiple queues can be provided by GPUs.
		let queue = try!(physical.queue_families()
			.find(|q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
			.ok_or(format!("Failed to find graphics command queue")));
		
		// Initialise device
		let (device, queues) = try!(Device::new(
			&physical,
			physical.supported_features(),
			&DeviceExtensions::none(),
			&[],
			[(queue, 1.0)].iter().cloned()).map_err(|e| format!("Failed to create device: {}", e)));
		
		let queue = queues[0].clone();
		
		let (swapchain, images) = {
			let caps = try!(surface.get_capabilities(&physical).map_err(|e| format!("Failed to get surface capabilities: {}", e)));
			
			let dimensions = caps.current_extent.unwrap_or([600, 800]);
			let present = caps.present_modes[0];
			let usage = caps.supported_usage_flags;
			
			try!(Swapchain::new(&device, &surface, /*num_images*/3,
				format::R8G8B8A8Srgb, dimensions, /*layers*/0,
				&usage, &queue, SurfaceTransform::Identity, CompositeAlpha::Opaque,
				present, true).map_err(|e| format!("Failed to create swapchain: {}", e)))
		};
		
		let vertex_buffer = try!(
			Buffer::new::<[[f64; 2]; 3], _>(&device, buffer::Usage::all(), HostVisible, &queue));
		
		{
			let mut wlock = vertex_buffer.try_write().unwrap();
			wlock[0] = [ -0.5, -0.25];
			wlock[1] = [  0.0,  0.5];
			wlock[2] = [-0.25, -0.1];
		}
		
		let vs = try!(shaders::TestVert::load(&device).map_err(|e| format!("Failed to load shader: {}", e)));
		let fs = try!(shaders::TestFrag::load(&device).map_err(|e| format!("Failed to load shader: {}", e)));
		
		// Create command buffer pool
		let cb_pool = CommandBufferPool::new(&device, &queue.family());
		
		// Convert images to image views (???)
		let images = images.into_iter().map(|image| {
			let image = image.transition(image::Layout::PresentSrc, &cb_pool, &queue).unwrap();
			try!(ImageView::new(&image).map_err(|e| format!("Failed to create image view: {}", e)));
		}).collect();
		
		mod renderpass {
			single_pass_renderpass!{
				attachments: {
					color: {
						load: Clear,
						store: Store,
						format: R8G8B8A8Srgb,
					}
				},
				pass: {
					color: [color],
					depth_stencil: {}
				}
			}
		}
		
		let renderpass = try!(renderpass::CustomRenderPass::new(&device).map_err(|e| format!("Failed to create renderpass: {}", e)));
		
		let pipeline = {
			let ia = pipeline::input_assembly::InputAssembly::triangle_list();
			let raster = Default::default();
			let ms = pipeline::multisample::Multisample::disabled();
			let blend = vulkano::pipeline::blend::Blend {
				logic_op: None,
				blend_constants: Some([0.0; 4]),
			};
			
			let viewports = pipeline::viewport::ViewportsState::Fixed {
				data: vec![(
					pipeline::viewport::Viewport {
						origin: [0.0, 0.0],
						dimensions: [1244.0, 699.0],
						depth_range: 0.0 .. 1.0
					},
					vulkano::pipeline::viewport::Scissor {
						origin: [0, 0],
						dimensions: [1244, 699],
					}
				)]
			};
			
			pipeline::GraphicsPipeline::new(&device, &vs.main_entry_point(), &ia, &viewports,
											&raster, &ms, &blend, &fs.main_entry_point(),
											&PipelineLayout::new(&device, EmptyPipelineDesc, ()).unwrap(),
											renderpass.subpass(0).unwrap()).unwrap()
		};
		
		let framebuffers = images.iter().map(|image| {
			Framebuffer::new(&renderpass, (1244, 699, 1), (image.clone() as Arc<_>,)).unwrap()
		}).collect();
		
		Ok(Render{
			window: window,
			swapchain: swapchain,
			submissions: Vec::new(),
		})
	}
	
	pub fn event_loop(&self) {
		loop {
			// Clean submissions
			self.submissions.retain(|s| !s.destroying_would_block());
			
			// Aquire image from swapchain
			let image_num = self.swapchain.acquire_next_image(1000000).unwrap();
			
			// Present image
			self.swapchain.present(&self.queue, image_num).unwrap();
			
			// Push new commands to the command buffer
			self.submissions.push(vulkano::command_buffer::submit(&self.command_buffers[image_num], &self.queue).unwrap());
			
			// Poll events
			for e in self.window.poll_events() {
				match e {
					Event::Closed => break,
					_ => {},
				}
			}
		}
	}
}
