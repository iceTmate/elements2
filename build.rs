use std::process::{Command, exit};

fn compile_shader(shader_name: &str) {
	let glsl_path = format!("res/shader/{}{}", shader_name, ".glsl");
	let spv_path = format!("res/shader/{}{}", shader_name, ".spv");

	println!("{}{}", "cargo:rerun-if-changed=", glsl_path);
	// this line is usefull but unfortunately forces linkage of the app
	// println!("{}{}", "cargo:rerun-if-changed=", spv_path);

	let status = Command::new("glslangValidator")
		.arg("-V")
		.arg(glsl_path)
		.arg("-o").arg(spv_path)
		.status()
		.unwrap();

	match status.code() {
		Some(code) => if code != 0 { exit(code); },
		None => if !status.success() { exit(1); },
	}
}

fn main() {
	compile_shader("shader.vert");
	compile_shader("shader.frag");
}
