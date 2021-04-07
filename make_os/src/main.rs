#![no_std]
#![no_main]

use core::panic::PanicInfo;
//この関数はパニック時に呼ばれる
#[no_mangle]
pub extern "C" fn _start() -> !{
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}