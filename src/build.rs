extern crate vulkano_shaders;
extern crate glsl_to_spirv;

use std::io::{self, Read, Write};
use std::env;
use std::path::{Path, PathBuf};
use std::fs::{self, OpenOptions};

use glsl_to_spirv::ShaderType;

fn get_shader_type(path: &Path) -> Option<ShaderType> {
	match path.extension().and_then(|e| e.to_str()) {
		Some("vert") => Some(ShaderType::Vertex),
		Some("frag") => Some(ShaderType::Fragment),
		Some("geom") => Some(ShaderType::Geometry),
		Some("tesc") => Some(ShaderType::TessellationControl),
		Some("tese") => Some(ShaderType::TessellationEvaluation),
		Some("comp") => Some(ShaderType::Compute),
		_ => None,
	}
}

fn append_shader_type(name: &mut String, ty: ShaderType) {
	name.push_str(match ty {
		ShaderType::Vertex                 => "Vert",
		ShaderType::Fragment               => "Frag",
		ShaderType::Geometry               => "Geom",
		ShaderType::TessellationControl    => "TesControl",
		ShaderType::TessellationEvaluation => "TesEval",
		ShaderType::Compute                => "Comp",
	});
}

fn main() {
	// TODO: Change panics to exits.
	let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
	let shaders_in_dir = PathBuf::new().join(&manifest_dir).join("shaders");
	fs::create_dir_all(&shaders_in_dir).expect("Could not create shaders directory.");
	let shaders = fs::read_dir(&shaders_in_dir).unwrap()
			.map(|entry| entry.unwrap().path().to_path_buf())
			.map(|path| { let ty = get_shader_type(&path); (path, ty) })
			.filter_map(|(path, ty)|
				if let Some(ty) = ty {
					Some((path, ty))
				} else {
					writeln!(io::stderr(), "Warning: '{}' does not have a valid shader extension. Valid extensions: '.vert', '.frag', '.geom', '.tesc', '.tese' and '.comp'", path.display()).ok();
					None
				})
			.map(|(path, ty)| match OpenOptions::new().read(true).open(&path) {
				Ok(f) => {
					let mut name = path.with_extension("").file_name().unwrap().to_string_lossy().into_owned();
					append_shader_type(&mut name, ty.clone());
					(name, f, ty)
				},
				Err(e) => panic!("Could not open '{}': {}", path.display(), e),
			})
			.map(|(name, mut file, ty)| {
				let mut contents = String::new();
				match file.read_to_string(&mut contents) {
					Ok(_)  => (name, contents, ty),
					Err(e) =>  panic!("Could not read shader '{}': {}", name, e),
				}
			})
			.map(|(name, src, ty)| {
				match glsl_to_spirv::compile(&src, ty.clone()) {
					Ok(spirv) => {
						println!("Shader '{}' compiled.", name);
						(name, spirv)
					},
					Err(e)    => {
						writeln!(io::stderr(), "Error: Could not compile shader '{}':", name).ok();
						writeln!(io::stderr(), "{}", e).ok();
						panic!("Could not compile shader.");
					}
				}
			})
			.map(|(name, spirv)| {
				match vulkano_shaders::reflect(&name, spirv) {
					Ok(out) => (name, out),
					Err(e)  => {
						writeln!(io::stderr(), "Error: Could not compile shader into rust code '{}':", name).ok();
						writeln!(io::stderr(), "{:?}", e).ok();
						panic!("Could not compile shader.");
					}
				}
			});
	
	let shaders_dir = PathBuf::new().join(&manifest_dir).join("src/shaders/");
	fs::create_dir_all(&shaders_dir).expect("Could not create shaders directory.");
	
	for (name, out) in shaders {
		let path = shaders_dir.join(&name).with_extension("rs");
		println!("Writing shader to '{}'", path.display());
		let mut f = OpenOptions::new().write(true).create(true).truncate(true).open(path).expect("The shader file could not be opened");
		f.write_all(out.as_bytes()).expect("Could not open out file.");
	}
}
