*This is a reproduction of a post that I had written in 2016 on a blog that no longer exists.
The information found here is likely outdated, but it complements the code base.*

This post will guide the reader through the process of compiling working executables of programs written in Rust for MSP430 targets.

From the [Rust homepage](https://www.rust-lang.org/):

> Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
>
> (...)
>
> Featuring
>
> * zero-cost abstractions
> * move semantics
> * guaranteed memory safety
> * threads without data races
> * trait-based generics
> * pattern matching
> * type inference
> * minimal runtime
> * efficient C bindings

The first step will be to check LLVM's `msp430` backend, which is [considered highly experimental][1].
If that works and we can compile, link and run (at least simple) C programs on the hardware, the next step will be compiling from Rust.

From the [LLVM homepage](http://www.llvm.org/):

> The LLVM Project is a collection of modular and reusable compiler and toolchain technologies.

# Prerequisites

It is assumed that you have both LLVM and MSP430 GCC installed.

The sample program shown below will be used.
It blinks the yellow LED on the MSP-EXP430G2 launchpad (this is the first launchapd).

```C
#define WDTPW	0x5A00
#define WDTHOLD	0x0080

typedef unsigned char uint8_t;
typedef unsigned short int uint16_t;
typedef uint16_t size_t;

extern uint8_t P1OUT;
extern uint8_t P1DIR;
extern uint16_t WDTCTL;

int main(void)
{
	size_t i;

	WDTCTL = WDTPW | WDTHOLD;
	P1DIR = 0x40;

	while (1) {
		P1OUT ^= 0x40;
		for (i = 0; i &lt; 5000; i++);
	}
}
```

All commands shown below are excerpts from a Makefile,
that has the following additional definitions:

    CFLAGS = -std=c90 -Wpedantic
    DEVICE = msp430g2452

# Compiling with LLVM

The commandline interface that LLVM provides is very similar to what GCC offers.
What's different with LLVM is, while you have to compile GCC for each combination of host and target, every LLVM build includes all supported targets.

    clang --target=msp430 $(CFLAGS) src/main.c -o bin/clang-blink.elf

Turns out, the MSP430 backend does not include a code generator, yet.
Means, we can get assembly, but not objects.
Also means, we cannot rely on LLVM on its own, but have to use the GCC assembler and linker.

    clang -S --target=msp430 $(CFLAGS) src/main.c -o bin/clang-blink.s
    msp430-elf-gcc -mmcu=$(DEVICE) -L/opt/msp430/gcc/include/ -Wl,-Map=rpt/clang-msp430.map bin/clang-blink.s -o bin/clang-msp430.elf -e main

# Compiling and Linking Rust

The rust equivalent to above C code is shown below.

```rust
#![feature(lang_items)]

#![no_std]
#![no_main]

const WDTPW : u16 = 0x5A00;
const WDTHOLD : u16 = 0x0080;

extern {
    static P1IN: u8;
    static mut P1OUT: u8;
    static mut P1DIR: u8;
    /* watchdog */
    static mut WDTCTL: u16;
}

#[export_name = "main"]
pub fn start() {
    let mut i = 0;
    unsafe {
        /* watchdog off */
        WDTCTL = WDTPW | WDTHOLD;
        /* configure io */
        P1DIR = 0b01000001;
        P1OUT = 0b00000000;
    }

    loop {
        P1OUT ^= 0b01000000;
        i = 0;
        while i &lt; 5000 {             i += 1;         }     } } #[lang = "panic_fmt"] fn panic_fmt() -&gt; ! {
    loop {}
}

#[lang = "eh_personality"]
fn asdf() {}
```

Note how the above code uses the same symbols as the C code does?
They are provided by the linker script.

```
PROVIDE(P1IN  = 0x0020);
PROVIDE(P1OUT = 0x0021);
PROVIDE(P1DIR = 0x0022);
```

This is possible using an extern block and declaring the variables in there as static [as the reference tells us][2].

```
extern {
    static P1IN: u8;
    static mut P1OUT: u8;
    static mut P1DIR: u8;
}
```

Please note that static mutable variables are dangerous and can only be used in unsafe blocks.

To compile and link this code, run below commands.

    rustc --emit llvm-bc src/main.rs -o bin/rust-blink.ll
    clang -S --target=msp430 $(CFLAGS) bin/rust-blink.ll -o bin/rust-blink.s
    msp430-elf-gcc -mmcu=$(DEVICE) -L /opt/msp430/gcc/include/ -Wl,-Map=rpt/rust-msp430.map bin/rust-blink.s -o $@ -e main</pre>

# Panic!

Unfortunately, we're getting an error here:

    undefined reference to `core::panicking::panic::haf296e94ad32f436'

Rust is verifying the addition and will panic if something's not right.
This panic handling is declared in `libcore`.

We could use `#[no_core]` to get rid of that error, but that will also get rid of a *lot* of very basic implementations (bit operations etc.).
So this is no solution.

For other architectures, having the Rust compiler create a static library crate, is an option, too as that will persumably include the missing symbol(s).
But that's not an option here as well, because we need LLVM bytecode.

So, the easiest workaround is, to just export the missing symbol ourselves as described in [this StackOverflow answer][3].

```
#[export_name = "_ZN4core9panicking5panic17haf296e94ad32f436E"]
pub fn except_add_overflow() -&gt; ! {
 loop {}
}
```

Now that we have built the binary, it can be flashed and run on the target.
If you do, you will see the yellow LED (LED2) flash.

# References

[1]: https://raw.githubusercontent.com/llvm-mirror/llvm/ae5f5d3d3cdcd8a061cb67e9a30303242891b2a2/lib/Target/MSP430/README.txt "README for the MSP430 target in the LLVM repository"
[2]: https://doc.rust-lang.org/book/ffi.html#accessing-foreign-globals "Rust Documentation - FFI: Accessing Foreign Globals"
[3]: http://stackoverflow.com/questions/37929165/rust-and-c-linking-problems-with-minimal-program-and-no-std "Rust and C linking problems with minimal program and no_std"

also:
[Facepalm into clang and LLVM](ttp://blitzfunk.com/2014/03/28/facepalm-into-clang-llvm-and-llvm/)
