use std::env;
use std::path::PathBuf;

fn main() {
    let library = pkg_config::Config::new()
        .probe("libnotify")
        .expect("Cannot locate libnotify. Is it installed?");

    let paths = library
        .include_paths
        .iter()
        .map(|path| format!("-I{}", path.to_str().unwrap()))
        .collect::<Vec<_>>();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(paths)
        .generate()
        .expect("Unable to generate bindings");

    let out_path =
        PathBuf::from(env::var("OUT_DIR").expect("Out $OUT_DIR not set. Perhaps a bindgen error?"));

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write to bindings");
}
