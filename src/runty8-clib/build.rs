extern crate cbindgen;

use std::path::{Path, PathBuf};
use std::env;
use cbindgen::Config;

fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_dir = target_dir();//env::var("OUT_DIR").expect("No build target dir");

    // for (key, value) in env::vars() {
    //     println!("{key}: {value}");
    // }
    // println!("WQAT");

    let config = Config::from_file(crate_dir.join("cbindgen.toml")).expect("Unable to get config");
    cbindgen::generate_with_config(crate_dir, config)
    // cbindgen::Builder::new()
    //   .with_crate(crate_dir)
      // .generate()
      .expect("Unable to generate bindings")
      .write_to_file(&target_dir.join("runty8-clib.h"));
}

fn target_dir() -> PathBuf {
    let target = env::var("OUT_DIR").expect("No OUT_DIR set.");
    let mut path = PathBuf::from(target);
    path.pop();
    path.pop();
    path.pop();
    path
}
