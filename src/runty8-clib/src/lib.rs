#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;
// use alloc::vec::Vec;
use core::alloc::Layout;
use core::panic::PanicInfo;
// use cortex_m_rt::entry;
use embedded_alloc::Heap;
use core::ffi::{c_int, c_uint};

#[global_allocator]
static HEAP: Heap = Heap::empty();

// #[entry]
#[no_mangle]
pub extern "C" fn init() {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    // let mut xs = Vec::new();
    // xs.push(1);

    // loop { /* .. */ }
}

#[no_mangle]
pub extern "C" fn getPixel(x : c_int, y : c_int) -> c_uint {
    0
}

#[no_mangle]
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
pub extern "C" fn setInput(buttons : u8) {
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
