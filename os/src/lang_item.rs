use core::panic::PanicInfo;

// panic feature to be implemented
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}