MSP430 Blinking LED with Rust
==============================

This project is a simple example for programming MSP430 micro-controllers using Rust.

The Timer A0 is configured in "Up" mode in order to generate periodic interrupts.  In the ISR LED2 at P1.6 (MSP-EXPG2) will be toggled.


Prerequisites
--------------

You need a Rust nightly version (for `no-core`), clang and the msp430-elf GCC tool-chain.


Running
--------

To actually run the program you need `mspdebug` and have to write the address of the ISR into address `0xfff2`.

    prog bin/msp430g2452.elf
    mw 0xfff2 44 e1
    run


For more information, see [this blog post](https://aboutembedded.wordpress.com/2016/11/16/rust-for-msp430/).

