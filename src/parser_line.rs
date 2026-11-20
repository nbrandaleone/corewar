use crate::types::{Mode, Modifier, Opcode};

pub fn parse_opcode(s: &str) -> Option<Opcode> {
    match s.to_ascii_uppercase().as_str() {
        "DAT" => Some(Opcode::Dat),
        "MOV" => Some(Opcode::Mov),
        "ADD" => Some(Opcode::Add),
        "SUB" => Some(Opcode::Sub),
        "MUL" => Some(Opcode::Mul),
        "DIV" => Some(Opcode::Div),
        "MOD" => Some(Opcode::Mod),
        "JMP" => Some(Opcode::Jmp),
        "JMZ" => Some(Opcode::Jmz),
        "JMN" => Some(Opcode::Jmn),
        "DJN" => Some(Opcode::Djn),
        "SPL" => Some(Opcode::Spl),
        "CMP" | "SEQ" => Some(Opcode::Seq),
        "SNE" => Some(Opcode::Sne),
        "SLT" => Some(Opcode::Slt),
        "NOP" => Some(Opcode::Nop),
        _ => None,
    }
}

pub fn parse_modifier(s: &str) -> Option<Modifier> {
    match s.to_ascii_uppercase().as_str() {
        "A" => Some(Modifier::A),
        "B" => Some(Modifier::B),
        "AB" => Some(Modifier::Ab),
        "BA" => Some(Modifier::Ba),
        "F" => Some(Modifier::F),
        "X" => Some(Modifier::X),
        "I" => Some(Modifier::I),
        _ => None,
    }
}

pub fn default_modifier(op: Opcode, a_mode: Mode, b_mode: Mode) -> Modifier {
    match op {
        Opcode::Dat | Opcode::Nop => Modifier::F,
        Opcode::Mov | Opcode::Seq | Opcode::Sne => {
            if a_mode == Mode::Immediate {
                Modifier::Ab
            } else if b_mode == Mode::Immediate {
                Modifier::B
            } else {
                Modifier::I
            }
        }
        Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div | Opcode::Mod => {
            if a_mode == Mode::Immediate {
                Modifier::Ab
            } else if b_mode == Mode::Immediate {
                Modifier::B
            } else {
                Modifier::F
            }
        }
        Opcode::Slt => {
            if a_mode == Mode::Immediate {
                Modifier::Ab
            } else {
                Modifier::B
            }
        }
        Opcode::Jmp | Opcode::Jmz | Opcode::Jmn | Opcode::Djn | Opcode::Spl => Modifier::B,
    }
}

pub fn split_mode_and_expr(operand: &str) -> (Mode, &str) {
    let trimmed = operand.trim();
    if let Some(first) = trimmed.chars().next() {
        let mode = match first {
            '#' => Some(Mode::Immediate),
            '$' => Some(Mode::Direct),
            '*' => Some(Mode::AIndirect),
            '@' => Some(Mode::BIndirect),
            '{' => Some(Mode::APreDec),
            '<' => Some(Mode::BPreDec),
            '}' => Some(Mode::APostInc),
            '>' => Some(Mode::BPostInc),
            _ => None,
        };
        if let Some(m) = mode {
            return (m, trimmed[first.len_utf8()..].trim());
        }
    }
    (Mode::Direct, trimmed)
}

pub fn is_pseudo_op(s: &str) -> bool {
    matches!(
        s.to_ascii_uppercase().as_str(),
        "ORG" | "END" | "EQU" | "PIN"
    )
}

pub fn is_keyword(s: &str) -> bool {
    parse_opcode(s).is_some() || is_pseudo_op(s)
}
