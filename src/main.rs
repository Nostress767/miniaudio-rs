#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
mod bindings;

use std::{
    env,
    ptr,
    mem::MaybeUninit,
    io::{ self, Read }
};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let mut engine = MaybeUninit::uninit();
    let engine_ptr = engine.as_mut_ptr();

    if args.len() < 2 {
        panic!("No input file.");
    }

    let result = unsafe { bindings::ma_engine_init(ptr::null(), engine_ptr) };
    if result != bindings::ma_result_MA_SUCCESS {
        panic!("Failed to initialize audio engine.");
    }

    args[1].push('\0');
    unsafe{ bindings::ma_engine_play_sound(engine_ptr, args[1].as_ptr() as *const i8, ptr::null_mut()) };

    println!("Press Enter to quit...");
    io::stdin().read(&mut [0]).unwrap();

    unsafe{ bindings::ma_engine_uninit(engine_ptr) };
}
