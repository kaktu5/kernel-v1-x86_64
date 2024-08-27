#![no_std]
#![no_main]
#![feature(abi_x86_interrupt, const_mut_refs)]

mod allocator;
mod gdt;
mod interrupts;
mod memory;
mod vga_buffer;

extern crate alloc;
use crate::memory::BootInfoFrameAllocator;
#[allow(unused_imports)]
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::{instructions, VirtAddr};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);

    println!("Hello, World!");

    hlt_loop();
}

fn init(boot_info: &'static BootInfo) {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    instructions::interrupts::enable();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();
}

fn hlt_loop() -> ! {
    loop {
        instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[allow(dead_code, unconditional_recursion)]
fn stack_overflow() {
    loop {
        stack_overflow();
    }
}
