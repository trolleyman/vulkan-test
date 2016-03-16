#[macro_use]
extern crate vulkano;
extern crate vulkano_shaders;
extern crate glutin;

use std::process::exit;
use std::io::{self, Write};

mod shaders;
mod render;

use render::Render;

fn main() {
	let r = match Render::new() {
		Ok(r)  => r,
		Err(e) => {
			writeln!(io::stderr(), "Error intialising renderer: {}", e);
			exit(1);
		},
	};
	
	r.event_loop();
	//let swapchain = Swapchain::new(&device, &surface, 2, );
}
