use crate::mars_math::to_signed;
use crate::types::{Instruction, Mode, Modifier, Opcode};

pub fn opcode_str(op: Opcode) -> &'static str {
    match op {
        Opcode::Dat => "DAT",
        Opcode::Mov => "MOV",
        Opcode::Add => "ADD",
        Opcode::Sub => "SUB",
        Opcode::Mul => "MUL",
        Opcode::Div => "DIV",
        Opcode::Mod => "MOD",
        Opcode::Jmp => "JMP",
        Opcode::Jmz => "JMZ",
        Opcode::Jmn => "JMN",
        Opcode::Djn => "DJN",
        Opcode::Spl => "SPL",
        Opcode::Seq => "SEQ",
        Opcode::Sne => "SNE",
        Opcode::Slt => "SLT",
        Opcode::Nop => "NOP",
    }
}

pub fn modifier_str(m: Modifier) -> &'static str {
    match m {
        Modifier::A => "A",
        Modifier::B => "B",
        Modifier::Ab => "AB",
        Modifier::Ba => "BA",
        Modifier::F => "F",
        Modifier::X => "X",
        Modifier::I => "I",
    }
}

pub fn mode_char(m: Mode) -> char {
    match m {
        Mode::Immediate => '#',
        Mode::Direct => '$',
        Mode::AIndirect => '*',
        Mode::BIndirect => '@',
        Mode::APreDec => '{',
        Mode::BPreDec => '<',
        Mode::APostInc => '}',
        Mode::BPostInc => '>',
    }
}

pub fn disassemble(inst: &Instruction, core_size: u32) -> String {
    let a_s = to_signed(inst.a_val, core_size);
    let b_s = to_signed(inst.b_val, core_size);
    format!(
        "{}.{:<2} {}{}, {}{}",
        opcode_str(inst.opcode),
        modifier_str(inst.modifier),
        mode_char(inst.a_mode),
        a_s,
        mode_char(inst.b_mode),
        b_s
    )
}
