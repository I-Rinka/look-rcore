#![no_std]
#![no_main]
use core::arch::global_asm;
global_asm!(include_str!("entry.asm")); // include_str -> read string from file -> global_asm embed it into the code

mod console;
mod lang_item;
mod sbi;

#[no_mangle] // see https://www.pwnthebox.net/rust/2020/11/01/deciphering-no-mangle.html; mangle will make the function name unreadable
fn rust_main() {
    // because we didn't clear the bss stack in asm, so we need to clear the bss here:
    clear_bss();
    println!("Hello rCore!");
    loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss(); // get the start of bss address
        fn ebss(); // get the end of bss address
    }
    // convert bss to raw pointer
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
