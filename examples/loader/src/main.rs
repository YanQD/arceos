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

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

fn abi_terminate(exit_code: i32) {
    println!("[ABI:Terminate] Terminate Apps by exit_code: {exit_code}!");
    exit(exit_code);
}

 static mut ABI_TABLE: [usize; 16] = [0; 16];

fn abi_entry(abi_num: usize, arg0: usize) {
    match abi_num  {
        SYS_HELLO => abi_hello(),
        SYS_PUTCHAR => abi_putchar(arg0 as u8 as char),
        SYS_TERMINATE => abi_terminate(arg0 as i32),
        _ => panic!("[ABI:Unknown] Unknown ABI: {abi_num}")
    }
}

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let app_num = 1;

    let mut app_start = PLASH_START as *const u8;
    
    println!("Load {app_num} app to payload...\n");

    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_terminate as usize);

    (0..app_num).for_each(|i| {
        let app_header = unsafe { app_start.add(2) } as *mut u8;
        let app_size: usize = ((unsafe { *app_start } as usize) << 8) + (unsafe { *app_start.add(1) } as usize);
        let content = unsafe { core::slice::from_raw_parts_mut(app_header, app_size)};

        // println!("App_Size {}, Content {:?}!", app_size, content);

        let run_code = unsafe {
            core::slice::from_raw_parts_mut(RUN_START as *mut u8, app_size)
        };

        run_code.copy_from_slice(content);

        println!("Execute app {i} ...");

        // execute app
        unsafe { core::arch::asm!("

        
            la      a0, {abi_entry}
            li      t2, {run_start}
            jalr    t2",      
            run_start = const RUN_START,
            abi_entry = sym abi_entry,
        )}
        app_start = unsafe { app_start.add(app_size + 1) };
    });

    println!("Load {app_num} app to payload ok!");
}