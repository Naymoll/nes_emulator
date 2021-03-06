use crate::nes::cpu::AddressingMode;

#[allow(dead_code)]
pub struct Instruction {
    pub opcode: u8,
    pub len: u8,
    pub cycle: u8,
    pub addressing_mode: AddressingMode,
}

#[allow(dead_code)]
impl Instruction {
    pub const fn new(opcode: u8, len: u8, cycle: u8, addressing_mode: AddressingMode) -> Self {
        Instruction {
            opcode,
            len,
            cycle,
            addressing_mode,
        }
    }

    pub fn from_code(code: u8) -> Self {
        match code {
            //ADC
            0x69 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0x65 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x75 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x6D => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0x7D => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0x79 => Instruction::new(code, 3, 4, AddressingMode::AbsoluteY),
            0x61 => Instruction::new(code, 2, 6, AddressingMode::IndirectX),
            0x71 => Instruction::new(code, 2, 5, AddressingMode::IndirectY),

            //AND
            0x29 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0x25 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x35 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x2D => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0x3D => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0x39 => Instruction::new(code, 3, 4, AddressingMode::AbsoluteY),
            0x21 => Instruction::new(code, 2, 6, AddressingMode::IndirectX),
            0x31 => Instruction::new(code, 2, 5, AddressingMode::IndirectY),

            //AHX
            0x93 => Instruction::new(code, 2, 8, AddressingMode::IndirectY),
            0x9F => Instruction::new(code, 3, 4, AddressingMode::AbsoluteY),

            //ALR
            0x4B => Instruction::new(code, 2, 2, AddressingMode::Immediate),

            //ANC
            0x0B => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0x2B => Instruction::new(code, 2, 2, AddressingMode::Immediate),

            //ARR
            0x6B => Instruction::new(code, 2, 2, AddressingMode::Immediate),

            //ASL
            0x0A => Instruction::new(code, 1, 2, AddressingMode::Accumulator),
            0x06 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0x16 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x0E => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0x1E => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),

            //AXS
            0xCB => Instruction::new(code, 2, 2, AddressingMode::Immediate),

            //BCC
            0x90 => Instruction::new(code, 2, 2, AddressingMode::Relative),
            //BCS
            0xB0 => Instruction::new(code, 2, 2, AddressingMode::Relative),
            //BEQ
            0xF0 => Instruction::new(code, 2, 2, AddressingMode::Relative),

            //BIT
            0x24 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x2C => Instruction::new(code, 3, 4, AddressingMode::Absolute),

            //BMI
            0x30 => Instruction::new(code, 2, 2, AddressingMode::Relative),
            //BNE
            0xD0 => Instruction::new(code, 2, 2, AddressingMode::Relative),
            //BPL
            0x10 => Instruction::new(code, 2, 2, AddressingMode::Relative),
            //BRK
            0x00 => Instruction::new(code, 1, 7, AddressingMode::Implied),
            //BVC
            0x50 => Instruction::new(code, 2, 2, AddressingMode::Relative),
            //BVS
            0x70 => Instruction::new(code, 2, 2, AddressingMode::Relative),

            //CLC
            0x18 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //CLD
            0xD8 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //CLI
            0x58 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //CLV
            0xB8 => Instruction::new(code, 1, 2, AddressingMode::Implied),

            //CMP
            0xC9 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0xC5 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0xD5 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0xCD => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0xDD => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0xD9 => Instruction::new(code, 3, 4, AddressingMode::AbsoluteY),
            0xC1 => Instruction::new(code, 2, 6, AddressingMode::IndirectX),
            0xD1 => Instruction::new(code, 2, 5, AddressingMode::IndirectY),

            //CPX
            0xE0 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0xE4 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0xEC => Instruction::new(code, 3, 4, AddressingMode::Absolute),

            //CPY
            0xC0 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0xC4 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0xCC => Instruction::new(code, 3, 4, AddressingMode::Absolute),

            //DCP
            0xC7 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0xD7 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0xCF => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0xDF => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),
            0xDB => Instruction::new(code, 3, 7, AddressingMode::AbsoluteY),
            0xD3 => Instruction::new(code, 2, 8, AddressingMode::IndirectY),
            0xC3 => Instruction::new(code, 2, 8, AddressingMode::IndirectX),

            //DEC
            0xC6 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0xD6 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0xCE => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0xDE => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),

            //DEX
            0xCA => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //DEY
            0x88 => Instruction::new(code, 1, 2, AddressingMode::Implied),

            //EOR
            0x49 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0x45 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x55 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x4D => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0x5D => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0x59 => Instruction::new(code, 3, 4, AddressingMode::AbsoluteY),
            0x41 => Instruction::new(code, 2, 6, AddressingMode::IndirectX),
            0x51 => Instruction::new(code, 2, 5, AddressingMode::IndirectY),

            //INC
            0xE6 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0xF6 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0xEE => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0xFE => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),

            //INX
            0xE8 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //INY
            0xC8 => Instruction::new(code, 1, 2, AddressingMode::Implied),

            //ISC
            0xE7 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0xF7 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0xEF => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0xFF => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),
            0xFB => Instruction::new(code, 3, 7, AddressingMode::AbsoluteY),
            0xE3 => Instruction::new(code, 2, 8, AddressingMode::IndirectX),
            0xF3 => Instruction::new(code, 2, 8, AddressingMode::IndirectY),

            //JMP
            0x4C => Instruction::new(code, 3, 3, AddressingMode::Absolute),
            0x6C => Instruction::new(code, 3, 5, AddressingMode::Indirect),

            //JSR
            0x20 => Instruction::new(code, 3, 6, AddressingMode::Absolute),

            //KIL
            0x02 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0x12 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0x22 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0x32 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0x42 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0x52 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0x62 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0x72 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0x92 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0xB2 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0xD2 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0xF2 => Instruction::new(code, 1, 2, AddressingMode::Implied),

            //LAS
            0xBB => Instruction::new(code, 3, 2, AddressingMode::AbsoluteY),

            //LAX - LDA + LDX
            0xA7 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0xB7 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageY),
            0xAF => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0xBF => Instruction::new(code, 3, 4, AddressingMode::AbsoluteY),
            0xA3 => Instruction::new(code, 2, 6, AddressingMode::IndirectX),
            0xB3 => Instruction::new(code, 2, 5, AddressingMode::IndirectY),

            //LAX - LDA + TAX
            0xAB => Instruction::new(code, 2, 3, AddressingMode::Immediate),

            //LDA
            0xA9 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0xA5 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0xB5 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0xAD => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0xBD => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0xB9 => Instruction::new(code, 3, 4, AddressingMode::AbsoluteY),
            0xA1 => Instruction::new(code, 2, 6, AddressingMode::IndirectX),
            0xB1 => Instruction::new(code, 2, 5, AddressingMode::IndirectY),

            //LDX
            0xA2 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0xA6 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0xB6 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageY),
            0xAE => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0xBE => Instruction::new(code, 3, 4, AddressingMode::AbsoluteY),

            //LDY
            0xA0 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0xA4 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0xB4 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0xAC => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0xBC => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),

            //LSR
            0x4A => Instruction::new(code, 1, 2, AddressingMode::Accumulator),
            0x46 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0x56 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x4E => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0x5E => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),

            //NOP
            0xEA => Instruction::new(code, 1, 2, AddressingMode::Implied),

            //NOP unofficial
            0x80 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0x82 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0x89 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0xC2 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0xE2 => Instruction::new(code, 2, 2, AddressingMode::Immediate),

            0x04 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x44 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x64 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x14 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x34 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x54 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x74 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0xD4 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0xF4 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x0C => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0x1C => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0x3C => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0x5C => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0x7C => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0xDC => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0xFC => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),

            0x1A => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0x3A => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0x5A => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0x7A => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0xDA => Instruction::new(code, 1, 2, AddressingMode::Implied),
            0xFA => Instruction::new(code, 1, 2, AddressingMode::Implied),

            //ORA
            0x09 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0x05 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x15 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x0D => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0x1D => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0x19 => Instruction::new(code, 3, 4, AddressingMode::AbsoluteY),
            0x01 => Instruction::new(code, 2, 6, AddressingMode::IndirectX),
            0x11 => Instruction::new(code, 2, 5, AddressingMode::IndirectY),

            //PHA
            0x48 => Instruction::new(code, 1, 3, AddressingMode::Implied),
            //PHP
            0x08 => Instruction::new(code, 1, 3, AddressingMode::Implied),
            //PLA
            0x68 => Instruction::new(code, 1, 4, AddressingMode::Implied),
            //PLP
            0x28 => Instruction::new(code, 1, 4, AddressingMode::Implied),

            //RLA
            0x27 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0x37 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x2F => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0x3F => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),
            0x3B => Instruction::new(code, 3, 7, AddressingMode::AbsoluteY),
            0x33 => Instruction::new(code, 2, 8, AddressingMode::IndirectY),
            0x23 => Instruction::new(code, 2, 8, AddressingMode::IndirectX),

            //ROL
            0x2A => Instruction::new(code, 1, 2, AddressingMode::Accumulator),
            0x26 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0x36 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x2E => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0x3E => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),

            //ROR
            0x6A => Instruction::new(code, 1, 2, AddressingMode::Accumulator),
            0x66 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0x76 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x6E => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0x7E => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),

            //RRA
            0x67 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0x77 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x6F => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0x7F => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),
            0x7B => Instruction::new(code, 3, 7, AddressingMode::AbsoluteY),
            0x63 => Instruction::new(code, 2, 8, AddressingMode::IndirectX),
            0x73 => Instruction::new(code, 2, 8, AddressingMode::IndirectY),

            //RTI
            0x40 => Instruction::new(code, 1, 6, AddressingMode::Implied),
            //RTS
            0x60 => Instruction::new(code, 1, 6, AddressingMode::Implied),

            //SAX
            0x87 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x97 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageY),
            0x8F => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0x83 => Instruction::new(code, 2, 6, AddressingMode::IndirectX),

            //SBC
            0xE9 => Instruction::new(code, 2, 2, AddressingMode::Immediate),
            0xE5 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0xF5 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0xED => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0xFD => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),
            0xF9 => Instruction::new(code, 3, 4, AddressingMode::AbsoluteY),
            0xE1 => Instruction::new(code, 2, 6, AddressingMode::IndirectX),
            0xF1 => Instruction::new(code, 2, 5, AddressingMode::IndirectY),

            //SBC unofficial
            0xEB => Instruction::new(code, 2, 2, AddressingMode::Immediate),

            //SEC
            0x38 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //SED
            0xF8 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //SEI
            0x78 => Instruction::new(code, 1, 2, AddressingMode::Implied),

            //SHX
            0x9E => Instruction::new(code, 3, 4, AddressingMode::AbsoluteY),
            //SHY
            0x9C => Instruction::new(code, 3, 4, AddressingMode::AbsoluteX),

            //SLO
            0x07 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0x17 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x0F => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0x1F => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),
            0x1B => Instruction::new(code, 3, 7, AddressingMode::AbsoluteY),
            0x03 => Instruction::new(code, 2, 8, AddressingMode::IndirectX),
            0x13 => Instruction::new(code, 2, 8, AddressingMode::IndirectY),

            //SRE
            0x47 => Instruction::new(code, 2, 5, AddressingMode::ZeroPage),
            0x57 => Instruction::new(code, 2, 6, AddressingMode::ZeroPageX),
            0x4F => Instruction::new(code, 3, 6, AddressingMode::Absolute),
            0x5F => Instruction::new(code, 3, 7, AddressingMode::AbsoluteX),
            0x5B => Instruction::new(code, 3, 7, AddressingMode::AbsoluteY),
            0x43 => Instruction::new(code, 2, 8, AddressingMode::IndirectX),
            0x53 => Instruction::new(code, 2, 8, AddressingMode::IndirectY),

            //STA
            0x85 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x95 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x8D => Instruction::new(code, 3, 4, AddressingMode::Absolute),
            0x9D => Instruction::new(code, 3, 5, AddressingMode::AbsoluteX),
            0x99 => Instruction::new(code, 3, 5, AddressingMode::AbsoluteY),
            0x81 => Instruction::new(code, 2, 6, AddressingMode::IndirectX),
            0x91 => Instruction::new(code, 2, 6, AddressingMode::IndirectY),

            //STX
            0x86 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x96 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x8E => Instruction::new(code, 3, 4, AddressingMode::Absolute),

            //STY
            0x84 => Instruction::new(code, 2, 3, AddressingMode::ZeroPage),
            0x94 => Instruction::new(code, 2, 4, AddressingMode::ZeroPageX),
            0x8C => Instruction::new(code, 3, 4, AddressingMode::Absolute),

            //TAS
            0x9B => Instruction::new(code, 3, 2, AddressingMode::AbsoluteY),

            //TAX
            0xAA => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //TAY
            0xA8 => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //TSX
            0xBA => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //TXA
            0x8A => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //TXS
            0x9A => Instruction::new(code, 1, 2, AddressingMode::Implied),
            //TYA
            0x98 => Instruction::new(code, 1, 2, AddressingMode::Implied),

            //XAA
            0x8B => Instruction::new(code, 2, 3, AddressingMode::Immediate),

            _ => unimplemented!("That code unimplemented"),
        }
    }
}
