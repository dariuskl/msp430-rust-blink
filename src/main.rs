#![feature(lang_items)]
#![feature(asm)]
#![feature(naked_functions)]

#![no_std]
#![no_main]

/// @file blink/src/main.c
/// @author Darius Kellermann <kellermann@protonmail.com>

const WDTPW         : u16   =   0x5A00;
const WDTHOLD       : u16   =   0x0080;

/// use SMCLK
const TASSEL_2      : u16   =   0b0000001000000000;
/// timer/counter continuously runs upwards to TAnCCRx
const MC_1          : u16   =   0b0000000000010000;
const CCIE          : u16   =   0b0000000000010000;

/* basic clock module */
const XT2OFF        : u8    =   0b10000000;
const RSEL0         : u8    =   0b00000001;

/* status register bits */
const GIE           : u8    =   0b00001000;
const CPUOFF        : u8    =   0b00010000;

const LED2          : u8    =   0b01000000;

extern {
    /* port 1 io */
    //static P1IN: u8;
    static mut P1OUT: u8;
    static mut P1DIR: u8;
    /* timer a */
    static mut TA0CTL: u16;
    static mut TA0CCTL0: u16;
    static mut TA0CCR0: u16;
    /* watchdog */
    static mut WDTCTL: u16;
    /* basic clock module */
    static mut BCSCTL1: u8;
}

#[naked]
pub unsafe fn toggle_led2() {
    /* P1OUT ^= 0b01000000; */
    asm!("push  r12
          mov.b &0x0021,r12     ; P1OUT -> r12
          xor   #64,    r12     ; r12 ^ 0b01000000
          mov.b r12,    &0x0021 ; r12 -> P1OUT
          pop   r12
          reti");
}

/// set the given bits in the status register
fn bis_sr(val: u8) {
    unsafe {
        asm!("bis   $0,  SR"
             :
             : "r"(val));
    }
}

#[export_name = "main"]
pub unsafe fn start() {
    /* cpu @0.15 MHz */
    BCSCTL1 = XT2OFF | RSEL0;
    /* watchdog off */
    WDTCTL = WDTPW | WDTHOLD;
    /* configure io */
    P1OUT = 0;
    P1DIR = LED2;
    /* configure and start timer a */
    TA0CCR0 = 32000;
    TA0CCTL0 = CCIE;
    TA0CTL = TASSEL_2 | MC_1;

    /* enable interrupts and turn CPU off */
    bis_sr(GIE | CPUOFF);
}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[export_name = "_ZN4core9panicking5panic17haf296e94ad32f436E"]
pub fn except_add_overflow() -> ! {
    loop {}
}
