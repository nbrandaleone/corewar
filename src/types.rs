#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Dat = 0,
    Mov = 1,
    Add = 2,
    Sub = 3,
    Mul = 4,
    Div = 5,
    Mod = 6,
    Jmp = 7,
    Jmz = 8,
    Jmn = 9,
    Djn = 10,
    Spl = 11,
    Seq = 12,
    Sne = 13,
    Slt = 14,
    Nop = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Modifier {
    A = 0,
    B = 1,
    Ab = 2,
    Ba = 3,
    F = 4,
    X = 5,
    I = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    Immediate = 0, // #
    Direct = 1,    // $
    AIndirect = 2, // *
    BIndirect = 3, // @
    APreDec = 4,   // {
    BPreDec = 5,   // <
    APostInc = 6,  // }
    BPostInc = 7,  // >
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub modifier: Modifier,
    pub a_mode: Mode,
    pub a_val: u32,
    pub b_mode: Mode,
    pub b_val: u32,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            opcode: Opcode::Dat,
            modifier: Modifier::F,
            a_mode: Mode::Direct,
            a_val: 0,
            b_mode: Mode::Direct,
            b_val: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarriorProgram {
    pub name: String,
    pub author: String,
    pub strategy: String,
    pub org: usize,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarsConfig {
    pub core_size: u32,
    pub max_cycles: u32,
    pub max_processes: u32,
    pub max_length: u32,
    pub min_distance: u32,
}

impl Default for MarsConfig {
    fn default() -> Self {
        Self {
            core_size: 8000,
            max_cycles: 80000,
            max_processes: 8000,
            max_length: 100,
            min_distance: 100,
        }
    }
}
