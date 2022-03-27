#![feature(asm_experimental_arch)]
#![feature(naked_functions)]
#![feature(asm_sym)]
#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

/// UserApp Header Area
#[repr(C)]
struct UserAppHeader {
    start: unsafe extern "C" fn() -> !,
    security_code: u32,
}

#[no_mangle]
#[link_section = ".userapp_hdr"]
#[used]
static _USERAPP_HDR: UserAppHeader = UserAppHeader {
    start,
    security_code: 0x55aa55aa,
};

#[no_mangle]
#[naked]
unsafe extern "C" fn start() -> ! {
    unsafe {
        core::arch::asm!(
            "
                # Initialize .data
                mov #__sidata, r2
                mov #__sdata, r1
                mov #(__edata - __sdata), r3
                smovf

                # Initialize .bss
                mov #__sbss, r1
                mov #(__ebss - __sbss), r3
                mov #0, r2
                sstr

                # Set the stack pointers
                mvtc #_ustack_start, usp
                mvtc #_istack_start, isp

                mvtc #0x100, fpsw

                bra _{main}
            ",
            main = sym main,
            options(noreturn)
        );
    }
}

/// Port direction register
const PORTA_PDR: *mut u8 = 0x0008C00A as *mut u8;
/// Port mode register
const PORTA_PMR: *mut u8 = 0x0008C06A as *mut u8;
/// Port output data register
const PORTA_PODR: *mut u8 = 0x0008C02A as *mut u8;

#[no_mangle]
extern "C" fn main() -> ! {
    unsafe {
        // Use PA0 (LED on GR-CITRUS) as a GPIO port
        PORTA_PMR.write_volatile(PORTA_PMR.read_volatile() & !0b00000001);
        // Use PA0 as an output port
        PORTA_PDR.write_volatile(PORTA_PDR.read_volatile() | 0b00000001);
    }

    loop {
        unsafe {
            // Toggle PA0
            PORTA_PODR.write_volatile(PORTA_PODR.read_volatile() ^ 0b00000001);
        }

        // Wait for a bit
        for _ in 0..5 * 1024 * 1024 {
            unsafe { core::arch::asm!("") };
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
