# noentiendo

A modular framework for emulating retro computers.

| Supported Systems | Roadmap      |
| ----------------- | ------------ |
| Commodore PET     | Commodore 64 |
| Commodore VIC-20  | Apple IIe    |
|                   | Nintendo NES |

| Supported Platforms             | Roadmap                 |
| ------------------------------- | ----------------------- |
| Desktop (Linux, macOS, Windows) | Android (Native)        |
| Web (via WebAssembly)           | iOS (Native)            |
|                                 | Embedded (e.g., RP2040) |

A [`libnoentiendo::System`] consists of a 6502 CPU and some attached **memory**. All computer peripherals are exposed to the CPU over the memory interface (i.e., _memory-mapped I/O_).

A [`libnoentiendo::Memory`] implementation can be read from and written to, but it can also be polled for interrupts. This is used for the PIA, VIA, and other chips that interface over memory but also trigger interrupts.

A [`libnoentiendo::Platform`] consumes a system and runs it. Platforms provide access to the video output, keyboard input, system random number generator, and other details via a [`libnoentiendo::PlatformProvider`]. Some platforms run synchronously (taking over the thread) while others run asynchronously with the help of an event loop (such as when compiling to WASM).
