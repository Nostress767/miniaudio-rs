extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=miniaudio.c");

    Command::new("clang")
        .args([
            "-c", 
            "-Oz", 
            "-fno-math-errno", 
            "-fno-exceptions", 
            "-fno-rtti", 
            "-DMA_NO_DSOUND", 
            "-DMA_NO_WINMM",
            "-DMA_NO_JACK",
            "-DMA_NO_COREAUDIO",
            "-DMA_NO_SNDIO",
            "-DMA_NO_AUDIO4",
            "-DMA_NO_OSS",
            "-DMA_NO_AAUDIO",
            "-DMA_NO_OPENSL",
            "-DMA_NO_WEBAUDIO",
            "-DMA_NO_NULL",
            "-DMA_NO_WAV",
            "-DMA_NO_FLAC",
            "-DMA_NO_GENERATION",
            "-DMA_NO_ENCODING",
            "miniaudio.c"])
        .output()
        .expect("clang not available");
    Command::new("llvm-ar")
        .args(["-rc", "miniaudio.lib", "miniaudio.o"])
        .output()
        .expect("llvm-ar not available");

    println!("cargo:rustc-link-search={}", env::current_dir().unwrap().display());
    println!("cargo:rustc-link-lib=miniaudio");

    let bindings = bindgen::Builder::default()
        .header("miniaudio.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::current_dir().unwrap());
    bindings
        .write_to_file(out_path.join("src/bindings.rs"))
        .expect("Couldn't write bindings!");
}

