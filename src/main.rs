extern crate vulkano;
extern crate vulkano_shaders;
extern crate glutin;
#[cfg(windows)]
extern crate kernel32;

use std::sync::Arc;
use std::ptr;

use glutin::{Window};

use vulkano::instance::{InstanceExtensions, Instance, PhysicalDevice};
use vulkano::swapchain::Surface;

mod shaders;

#[cfg(windows)]
fn get_surface(instance: &Arc<Instance>, w: &Window) -> Arc<Surface> {
	use glutin::os::windows::WindowExt;
	unsafe {
		let hwnd = w.get_hwnd();
		let hinstance = kernel32::GetModuleHandleA(ptr::null());
	
		Surface::from_hwnd(instance, hinstance, hwnd).expect("failed to create surface")
	}
}

fn main() {
	let extensions = InstanceExtensions {
		khr_surface: true,
		//khr_display: true,
		//khr_display_swapchain: true,
		khr_win32_surface: true,
		.. InstanceExtensions::none()
	};
	let instance = Instance::new(None, &extensions, &[]).expect("failed to create instance");
	
	for (i, device) in PhysicalDevice::enumerate(&instance).enumerate() {
		println!("Device {}: {} (type {:?})", i, device.name(), device.ty());
	}
	
	let device = PhysicalDevice::enumerate(&instance).next().expect("Error: No devices found");
	println!("Using device: {} (type: {:?})", device.name(), device.ty());
	
	let window = glutin::WindowBuilder::new()
		.with_title("Vulkan Test".into())
		.build().unwrap();
	
	let surface = get_surface(&instance, &window);
	
	//let swapchain = Swapchain::new(&device, &surface, 2, );
	
	
	
}
