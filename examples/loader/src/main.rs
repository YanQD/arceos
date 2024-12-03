#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let mut apps_start = PLASH_START as *const u8;

    println!("Load payload ...");

    while unsafe {*(apps_start)} as usize > 0 {
        println!("App_Size: {}", unsafe{*apps_start});
        let code = unsafe { core::slice::from_raw_parts(apps_start.add(1), *(apps_start) as usize) };
        apps_start = unsafe { apps_start.add(*(apps_start) as usize + 1) };
        println!("content: {:?}: ", code);
    }

    println!("Load payload ok!");
}