extern crate bindgen;

use std::env;
use std::path::PathBuf;

const WIN_DEF_DART_SDK_PATH: &str = "C:\\Program Files\\Dart\\dart-sdk";

fn main() {
	let mut bindings_builder = bindgen::Builder::default().header("wrapper.h");

	let dartsdk_path: Option<PathBuf> = if let Some(path) = option_env!("BINDGEN_DART_SDK_PATH") {
		Some(PathBuf::from(path))
	} else if cfg!(windows) {
		Some(PathBuf::from(WIN_DEF_DART_SDK_PATH))
	} else {
		None
	};

	if let Some(path) = dartsdk_path {
		let path_include = path.join("include");
		let path_bin = path.join("bin");

		bindings_builder = bindings_builder.clang_arg(format!("-I{}", path_include.display()));
		println!("cargo:rustc-link-search={}", path_bin.display());
	}

	println!("cargo:rustc-link-lib=static=dart");

	let bindings = bindings_builder
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
