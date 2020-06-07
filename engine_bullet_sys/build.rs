use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
	let mut build = cc::Build::new();
	build.cpp(true)
		.shared_flag(true)
		.define("BT_THREADSAFE", "1")
		.define("BT_USE_DOUBLE_PRECISION", "1")
		.define("BT_CLAMP_VELOCITY_TO", "9999");
	
	// Include submodule include dirs
	let submodules_dir = PathBuf::from("bullet/src/");
	build.include(submodules_dir.into_os_string().into_string().expect("Invalid path"));
	
	// Include bullet submodules source files
	compile_submodule(build.clone(), "bullet/src/Bullet3Common", "bullet3_common")?;
	compile_submodule(build.clone(), "bullet/src/Bullet3Geometry", "bullet3_geometry")?;
	compile_submodule(build.clone(), "bullet/src/Bullet3Collision", "bullet3_collision")?;
	compile_submodule(build.clone(), "bullet/src/Bullet3Dynamics", "bullet3_dynamics")?;
//	include_submodule(&mut build, "bullet/src/Bullet3OpenCL")?;
//	include_submodule(&mut build, "bullet/src/Bullet3Serialize/Bullet2FileLoader")?;
	compile_submodule(build.clone(), "bullet/src/BulletInverseDynamics", "bullet_inverse_dynamics")?;
	compile_submodule(build.clone(), "bullet/src/BulletSoftBody", "bullet_softbody")?;
	compile_submodule(build.clone(), "bullet/src/BulletDynamics", "bullet_dynamics")?;
	compile_submodule(build.clone(), "bullet/src/BulletCollision", "bullet_collision")?;
	compile_submodule(build.clone(), "bullet/src/LinearMath", "bullet_linearmath")?;
	
//	// Gen bullet header bindings
//	let bindings = bindgen::builder()
//		.header("bullet/src/BulletDynamics/Dynamics/btDynamicsWorld.h")
//		.whitelist_function()
	
	Ok(())
}

fn compile_submodule(mut build: cc::Build, path: &str, out_name: &str) -> Result<(), Box<dyn Error>> {
//	// Include submodule dir
//	let submodule_dir = PathBuf::from(path);
//	build.include(submodule_dir.into_os_string().into_string().expect("Invalid include dir path"));
	
	// Add source files
	let src_file_path = path.to_string() + "/**/*.cpp";
	let src_file_iter = glob::glob(&src_file_path)?.filter_map(|p| p.ok());
	build.files(src_file_iter);
	
	// Compile
	build.try_compile(out_name).map_err(|e| e.into())
}
