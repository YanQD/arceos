#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

#[cfg(feature = "axstd")]
use axstd::{ println, process::exit };

const PLASH_START: usize = 0x22000000;

// app running aspace
// SBI(0x80000000) -> App <- Kernel(0x80200000)
// 0xffff_ffc0_0000_0000
const RUN_START: usize = 0xffff_ffc0_8010_0000;

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

fn abi_terminate() {
    println!("[ABI:Terminate] Terminate Apps!");
    exit(0);
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let app_num = 1;

    let mut app_start = PLASH_START as *const u8;
    
    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);

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

        println!("Execute app {i} ...");

        // execute app
        unsafe { core::arch::asm!("
            la      a7, {abi_table}
            li      t2, {run_start}
            jalr    t2",
            run_start = const RUN_START,
            abi_table = sym ABI_TABLE,
        )}

        app_start = unsafe { app_start.add(app_size + 1) };
    });

    println!("Load {app_num} app to payload ok!");


}