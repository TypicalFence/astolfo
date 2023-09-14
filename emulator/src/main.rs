mod cpu;


const BYTE_ARRAY: [u8; 12] = [
    0b00110011, 0b10010100, 0b00100011, 0b10010100,
    0b00100011, 0b10010100, 0b00100011, 0b00001100,
    0b00111010, 0b10010100, 0b00000000, 0b00000000,
];

fn main() {
    let mut prog: [u8; 32768] = [0; 32768];
    for byte in BYTE_ARRAY.iter().enumerate() {
        prog[byte.0] = *byte.1;
    }
    let mut cpu = cpu::Cpu::<32, 2048>::new();
    cpu.load_program(prog);
    for _ in 0..6 {
        cpu.tick();
        println!("{:?}", cpu.registers)
    }
}
