#![no_std]
#![no_main]
use core::arch::global_asm;
global_asm!(include_str!("entry.asm")); // include_str -> read string from file -> global_asm embed it into the code

mod lang_item;

fn main() {
    
}
