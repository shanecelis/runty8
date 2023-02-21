#![no_std]
#![feature(alloc_error_handler)]
use runty8_core::{App, Event, Input, Pico8, Resources, Map, Flags, SpriteSheet, Color};

// Need this to fix the link error where critical-section is undefined.
use cortex_m as _;

extern crate alloc;
// use alloc::vec::Vec;
use core::alloc::Layout;
use core::panic::PanicInfo;
// use cortex_m_rt::entry;
use embedded_alloc::Heap;
use core::ffi::{c_int, c_uint};
use crate::alloc::borrow::ToOwned;
use alloc::string::String;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub struct RuntyState {
    pico8 : Pico8,
    // game : Game,
    input : Input,
}

// #[no_mangle]
// pub extern "C" fn runty_new() -> *mut RuntyState {
//     opaque_pointer::raw(RuntyState { value: 0 })
// }

// #[entry]
#[no_mangle]
pub extern "C" fn init() -> *mut RuntyState {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    // let mut xs = Vec::new();
    // xs.push(1);
    //
    //
    // let resources = runty8_core::load_assets!("confetti").unwrap();
    // let assets_path = "hi".to_owned();
    // let sprite_sheet = SpriteSheet::default();
    // let sprite_flags = Flags::default();
    // let map = Map::default();

    // let resources = Resources {
    // assets_path,
    // sprite_sheet,
    // sprite_flags,
    // map,
    // };
    let resources : Resources = Default::default();

    let pico8 = Pico8::new(resources);
    // let mut game = Game::init(&mut pico8);
    let input = Input::new();

    opaque_pointer::raw(RuntyState { pico8, input })
}

#[no_mangle]
pub extern "C" fn getPixel(state : *const RuntyState, x : c_int, y : c_int) -> c_uint {
    return x as c_uint;
    let state = unsafe { opaque_pointer::object(state) };
    if state.is_err() {
        return x as c_uint;
    }
    let state = state.unwrap();
    let draw_data = &state.pico8.draw_data;
    draw_data.get_pixel(draw_data.index(x, y).unwrap_or(0))
}

#[repr(C)]
pub enum Button {
    W = 0b00000001,
    S = 0b00000010,
    A = 0b00000100,
    D = 0b00001000,
    I = 0b00010000,
    K = 0b00100000,
    J = 0b01000000,
    L = 0b10000000
}

#[no_mangle]
pub extern "C" fn setInput(state : *mut RuntyState, buttons : u8) {
}


// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// https://github.com/rust-embedded/embedded-alloc/blob/master/examples/global_alloc.rs
#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
