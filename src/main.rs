use chip8rs::Chip8;

fn main() {
    let program: Vec<u8> = vec![0x00, 0xE0, 0x10, 0x01];
    let len = program.len() / 2;
    let mut vm = Chip8::new(program);

    for _ in 0..len {
        vm.step();
    }
}
