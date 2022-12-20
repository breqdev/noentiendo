# Assembly Programs

The `.asm` files in this directory contain programs written for the 6502.

## How To Assemble

These files are written for the `vasm6502_oldstyle` assembler. Install it using the following:

```bash
wget http://sun.hasenbraten.de/vasm/release/vasm.tar.gz
tar xvzf vasm.tar.gz
cd vasm
make CPU=6502 SYNTAX=oldstyle
cp vasm6502_oldstyle /usr/bin
cp vobjdump /usr/bin
```

This script:

- Downloads the VASM source code and untars it
- Compiles the assembler for the 6502 processor and the "oldstyle" syntax
- Copies the assembler into `/usr/bin` for use

Compile with the following command:

```bash
vasm6502_oldstyle -Fbin -dotdir asm/program.s -o bin/program.bin
```

- `-Fbin`: Export in binary format
- `-dotdir`: Enable "dot-directives" (e.g. `.org`, `.word`)

## Listing

### `capitalize.s`

Written for the `brooke` system. Capitalizes each character which is inputted.

### `easy.s`

Adapted from the [easy6502](https://skilldrick.github.io/easy6502/#snake)
project's Snake demo to assemble with the `vasm6502_oldstyle` assembler.
Written for the `easy` system. Controls: W, A, S, D.

### `fake_pet_kernel.s`

Replaces the Commodore PET's kernal and simply writes "HELLO WORLD" to the
screen memory. Useful for testing.

### `fib.s`

Generates Fibonacci numbers. Written for the `brooke` system.

### `guessing.s`

Simple "too high" / "too low" / "you win" guessing game. Written for the `brooke` system.

### `multiply.s`

Implementation of the shift-and-add multiplication algorithm. Written for the `brooke` system.

### `syscall.s`

Example of using the `BRK` instruction for a syscall-like interface, as was done on the Apple IIe. Written for the `brooke` system.
