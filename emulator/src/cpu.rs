use astolfo_asm::instructions::Instruction;
use astolfo_asm::instructions::parse_binary;

#[derive(Debug)]
pub(crate) struct Cpu<const REGISTERS: usize, const SRAM: usize> {
    pub registers: [u8; REGISTERS],
    status_register: StatusRegister,
    eeprom: [u8; SRAM],
    flash: [u8; 32768],
    program_counter: u16,
}

#[derive(Debug)]
struct StatusRegister {
    carry: bool,
    zero: bool,
    negative: bool,
    overflow: bool,
    sign: bool,
    half_carry: bool,
    twos_complement: bool,
}

impl<const REGS: usize, const SRAM: usize> Cpu<REGS, SRAM> {
    pub fn new() -> Self {
        Self {
        registers: [0; REGS],
            status_register: StatusRegister {
                carry: false,
                zero: false,
                negative: false,
                overflow: false,
                sign: false,
                half_carry: false,
                twos_complement: false,
            },
            eeprom: [0; SRAM],
            flash: [0; 32768],
            program_counter: 0,
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Inc(reg) => {
                self.registers[reg as usize] += 1;
            }
            Instruction::Dec(reg) => {
                self.registers[reg as usize] -= 1;
            }
            Instruction::Add(rd, rr) => {
                self.registers[rd as usize] =  self.registers[rd as usize] + self.registers[rr as usize];
            }
            Instruction::Ldi(rd, k) => {
                // Ldi addresses r16-r32 with 0-15
                let reg = rd + 16;
                if reg <= 15 {
                    panic!("Invalid register, only register 16-32 are supported")
                }
                self.registers[reg as usize] =  k;
            }
            Instruction::Nop => {}
        }
    }

    pub fn tick(&mut self) {
        let instruction = self.fetch_instruction();
        println!("instruction: {:?}", instruction);
        self.execute_instruction(instruction);
        self.program_counter += 2; // always move 2 bytes;
    }

    fn fetch_instruction(&self) -> Instruction {
        let next_instruction = [
            self.flash[self.program_counter as usize],
            self.flash[self.program_counter as usize + 1],
            self.flash[self.program_counter as usize + 2],
            self.flash[self.program_counter as usize + 3],
        ];
        let instruction = parse_binary(next_instruction);
        match instruction {
            Some(i) => i,
            None => {
                panic!("Invalid instruction");
            },
        }

    }

    fn clear_flash(&mut self) {
        for byte in self.flash.iter_mut() {
            *byte = 0;
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        //assert!(program.len() > self.flash.len());
        self.clear_flash();
        for (i, byte) in program.iter().enumerate() {
            self.flash[i] = *byte;
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn inc() {
        let prog = include_bytes!("../test/inc.bin");
        let mut cpu = super::Cpu::<32, 2048>::new();;
        cpu.load_program(prog);
        cpu.tick();
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.registers[0], 2);
        assert_eq!(cpu.registers[1], 1);
    }

    #[test]
    fn dec() {
        let prog = include_bytes!("../test/dec.bin");
        let mut cpu = super::Cpu::<32, 2048>::new();;
        cpu.load_program(prog);
        cpu.registers[0] = 3;
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.registers[0], 1);
    }

    #[test]
    fn nop() {
        let prog = include_bytes!("../test/nop.bin");
        let mut cpu = super::Cpu::<4, 2048>::new();;
        cpu.load_program(prog);
        cpu.tick();
        assert_eq!(cpu.registers, [0, 0, 0, 0]);
    }


    #[test]
    fn add() {
        let prog = include_bytes!("../test/add.bin");
        let mut cpu = super::Cpu::<4, 2048>::new();;
        cpu.registers = [1,2, 0, 0];
        cpu.load_program(prog);
        cpu.tick();
        assert_eq!(cpu.registers, [3, 2, 0, 0]);
    }

    #[test]
    fn ldi() {
        let prog = include_bytes!("../test/ldi.bin");
        let mut cpu = super::Cpu::<32, 2048>::new();;
        cpu.load_program(prog);
        cpu.tick();
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.registers[16], 12);
        assert_eq!(cpu.registers[20], 40);
        assert_eq!(cpu.registers[24], 80);
    }
}