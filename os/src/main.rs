#![no_main]
#![no_std]
// 加上这个以后，才能通过 PanicInfo::message 获取报错信息
#![feature(panic_info_message)]

use core::arch::global_asm;

#[macro_use]
mod console;
mod lang_items;
mod sbi;

// 使用global_asm宏，将同目录下的汇编文件 entry.asm 嵌入到代码中。
global_asm!(include_str!("entry.asm"));


pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
} 