[![Rust](https://github.com/krueger71/chip8rs/actions/workflows/rust.yml/badge.svg)](https://github.com/krueger71/chip8rs/actions/workflows/rust.yml)

# chip8rs - Chip8 Emulator

This is a Chip8 emulator that uses SDL2 for video, audio and keyboard support.

The model for the Chip8 is defined in [chip8.rs](src/chip8.rs). The model is independent of the framework used for input and output.

In [emusdl2.rs](src/emusdl2.rs) the Chip8-model is connected to video, audio and keyboard using SDL2.

In [main.rs](src/main.rs) command line arguments are parsed and the emulator created and run.

The purpose of the implementation is both to learn Rust and basic emulator programming.

## References

- [https://en.wikipedia.org/wiki/CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
- [https://tonisagrista.com/blog/2021/chip8-spec/](https://tonisagrista.com/blog/2021/chip8-spec/)
- [http://devernay.free.fr/hacks/chip8/C8TECH10.HTM](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [https://chip-8.github.io/](https://chip-8.github.io/)
- [https://tobiasvl.github.io/blog/write-a-chip-8-emulator/](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [https://github.com/JohnEarnest/Octo](https://github.com/JohnEarnest/Octo)
- [https://github.com/mattmikolay/chip-8](https://github.com/mattmikolay/chip-8)
- [https://github.com/Timendus/chip8-test-suite](https://github.com/Timendus/chip8-test-suite)
