DEVICE = msp430g2452

LFLAGS = -L /opt/msp430/gcc/include/ -T $(DEVICE).ld

.PHONY: clean

all: bin/$(DEVICE).elf

bin/$(DEVICE).elf: src/main.rs Makefile
	rustc --emit llvm-bc src/main.rs -o bin/main.ll
	clang -S --target=msp430 bin/main.ll -o bin/main.s
	msp430-elf-gcc -mmcu=$(DEVICE) $(LFLAGS) -Wl,-Map=rpt/$(DEVICE).map bin/main.s -o $@ -e main
	msp430-elf-objdump -D $@ > rpt/$(DEVICE).s

debug:
	mspdebug rf2500

clean:
	rm -vf bin/* rpt/*
