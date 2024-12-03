#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;

// struct App {
//     size: usize,
//     data: Vec<u8>
// }

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;
    let apps_size = unsafe {*(apps_start)} as usize;

    let apps_start = unsafe { apps_start.add(1) };

    println!("Load payload ...");

    let code = unsafe { core::slice::from_raw_parts(apps_start, apps_size) };
    println!("content: {:?}: ", code);

    println!("Load payload ok!");
}