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

    pub(crate) fn load_program(&mut self, program: [u8; 32768]) {
        self.flash = program;
    }
}