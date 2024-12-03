#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0x22000000;

// app running aspace
// SBI(0x80000000) -> App <- Kernel(0x80200000)
// 0xffff_ffc0_0000_0000
const RUN_START: usize = 0xffff_ffc0_8010_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let app_num = 2;

    let mut app_start = PLASH_START as *const u8;
    
    // unsafe { *app_start };

    println!("Load {app_num} app to payload...\n");

    (0..app_num).for_each(|i| {
        let app_header = unsafe { app_start.add(1) } as *mut u8;
        let app_size = unsafe { *app_start } as usize;
        let content = unsafe { core::slice::from_raw_parts_mut(app_header, app_size)};

        println!("App_Size {}, Content {:?}", app_size, content);

        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, app_size)
        };

        run_code.copy_from_slice(content);
        println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());

        println!("Execute App_{i} ...");
        // execute app
        unsafe { core::arch::asm!("
            li      t2, {run_start}
            jalr    t2",
            run_start = const RUN_START,
        )}
        println!("Execute App_{i} done\n");

        app_start = unsafe { app_start.add(app_size + 1) };
    });

    println!("Load {app_num} app to payload ok!");
}