pub type Rd = u8;
pub type Rr = u8;

#[derive(Debug)]
pub enum Instruction {
    /// Increments the Register Reg
    ///
    /// [Atmel Documentation](https://ww1.microchip.com/downloads/en/devicedoc/atmel-0856-avr-instruction-set-manual.pdf#page=101)
    Inc(Rd),
    /// Decrements the Register Reg
    ///
    /// [Atmel Documentation](https://ww1.microchip.com/downloads/en/devicedoc/atmel-0856-avr-instruction-set-manual.pdf#page=84)
    Dec(Rd),
    /// Adds two registers without the C Flag and places the result in the destination register Rd.
    ///
    /// [Atmel Documentation](https://ww1.microchip.com/downloads/en/devicedoc/atmel-0856-avr-instruction-set-manual.pdf#page=32)
    Add(Rd, Rr),
    Nop,
}

/// instructions range from 16/32 bits
pub fn parse_binary(bytes: [u8; 4]) -> Option<Instruction> {
    let b1 = bytes[0];
    let b2 = bytes[1];

    let bits16 = ((b2 as u16) << 8) | (b1 as u16);

    return parse_16(bits16);
}

fn parse_16(bits: u16) -> Option<Instruction> {
    if bits == 0 {
        return Some(Instruction::Nop);
    } else if let Some(i) = parse_rd(bits) {
        return Some(i);
    }
    else if let Some(i) = parse_rdrr(bits) {
        return Some(i);
    }

    None
}

/// rd: `<|opcode|fffd|dddd|ffff|>`.
fn parse_rd(bits: u16) -> Option<Instruction> {
    let opcode =
                       bits & 0b1111_1110_0000_1111;

    let rd = (bits & 0b0000_0001_1111_0000) >> 4;
    let rdu8 =rd as u8;
    //println!("opcode: {:b}", opcode);
    //println!("rd: {}", rd);
    match opcode {
        0b1001_0100_0000_0011 => Some(Instruction::Inc(rdu8)),
        0b1001_0100_0000_1010 => Some(Instruction::Dec(rdu8)),

        _ => None,
    }
}

/// rdrr: `<|opcode|ffrd|dddd|rrrr|>`
fn parse_rdrr(bits: u16) -> Option<Instruction> {
    let opcode =
        bits & 0b1111_1100_0000_0000;

    let rd = (bits & 0b0000_0011_1111_0000) >> 4;
    let rr= (bits & 0b0000_0000_0001_1111);

    match opcode {
        0b0000_1100_0000_0000 => Some(Instruction::Add(rd as u8, rr as u8)),
        _ => None,
    }
}
