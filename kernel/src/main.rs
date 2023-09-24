#![no_std]
#![no_main]

// mod vga_buffer;
mod frame_buffer;

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};
use frame_buffer::FrameBufferWriter;
use spin::Mutex;

use crate::frame_buffer::WRITER;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

entry_point!(kernel_main);

static HELLO: &str = "Hello World!";

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info().clone();
        unsafe {
            WRITER = Mutex::new(Some(FrameBufferWriter::new(
                framebuffer.buffer_mut(),
                info
            )));
        }
    }
    println!("{}", HELLO);

    loop {}
}
