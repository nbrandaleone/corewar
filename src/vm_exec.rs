use crate::mars_math::{add_mod, dec_mod, mul_mod, sub_mod};
use crate::types::{Instruction, Modifier, Opcode};
use crate::vm_eval::EvalResult;

pub enum ExecAction {
    Advance(u32),
    Jump(u32),
    Split(u32, u32),
    Die,
}

pub struct ExecResult {
    pub action: ExecAction,
    pub wrote_addr: Option<u32>,
}

pub fn execute_inst(
    core: &mut [Instruction],
    pc: u32,
    ir: &Instruction,
    a: &EvalResult,
    b: &EvalResult,
) -> ExecResult {
    let n = core.len() as u32;
    let next_pc = add_mod(pc, 1, n);
    let skip_pc = add_mod(pc, 2, n);
    let dst = &mut core[b.addr as usize];

    match ir.opcode {
        Opcode::Dat => ExecResult { action: ExecAction::Die, wrote_addr: None },
        Opcode::Nop => ExecResult { action: ExecAction::Advance(next_pc), wrote_addr: None },
        Opcode::Jmp => ExecResult { action: ExecAction::Jump(a.addr), wrote_addr: None },
        Opcode::Spl => ExecResult { action: ExecAction::Split(next_pc, a.addr), wrote_addr: None },
        Opcode::Mov => {
            apply_mov(ir.modifier, &a.inst, dst);
            ExecResult { action: ExecAction::Advance(next_pc), wrote_addr: Some(b.addr) }
        }
        Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div | Opcode::Mod => {
            let ok = apply_arith(ir.opcode, ir.modifier, &a.inst, &b.inst, dst, n);
            if ok {
                ExecResult { action: ExecAction::Advance(next_pc), wrote_addr: Some(b.addr) }
            } else {
                ExecResult { action: ExecAction::Die, wrote_addr: None }
            }
        }
        Opcode::Jmz => {
            let zero = match ir.modifier {
                Modifier::A | Modifier::Ba => b.inst.a_val == 0,
                Modifier::B | Modifier::Ab => b.inst.b_val == 0,
                Modifier::F | Modifier::X | Modifier::I => b.inst.a_val == 0 && b.inst.b_val == 0,
            };
            let target = if zero { a.addr } else { next_pc };
            ExecResult { action: ExecAction::Jump(target), wrote_addr: None }
        }
        Opcode::Jmn => {
            let nz = match ir.modifier {
                Modifier::A | Modifier::Ba => b.inst.a_val != 0,
                Modifier::B | Modifier::Ab => b.inst.b_val != 0,
                Modifier::F | Modifier::X | Modifier::I => b.inst.a_val != 0 || b.inst.b_val != 0,
            };
            let target = if nz { a.addr } else { next_pc };
            ExecResult { action: ExecAction::Jump(target), wrote_addr: None }
        }
        Opcode::Djn => {
            let nz = match ir.modifier {
                Modifier::A | Modifier::Ba => {
                    dst.a_val = dec_mod(dst.a_val, n);
                    dst.a_val != 0
                }
                Modifier::B | Modifier::Ab => {
                    dst.b_val = dec_mod(dst.b_val, n);
                    dst.b_val != 0
                }
                Modifier::F | Modifier::X | Modifier::I => {
                    dst.a_val = dec_mod(dst.a_val, n);
                    dst.b_val = dec_mod(dst.b_val, n);
                    dst.a_val != 0 || dst.b_val != 0
                }
            };
            let target = if nz { a.addr } else { next_pc };
            ExecResult { action: ExecAction::Jump(target), wrote_addr: Some(b.addr) }
        }
        Opcode::Seq => {
            let eq = compare_eq(ir.modifier, &a.inst, &b.inst);
            ExecResult { action: ExecAction::Advance(if eq { skip_pc } else { next_pc }), wrote_addr: None }
        }
        Opcode::Sne => {
            let eq = compare_eq(ir.modifier, &a.inst, &b.inst);
            ExecResult { action: ExecAction::Advance(if !eq { skip_pc } else { next_pc }), wrote_addr: None }
        }
        Opcode::Slt => {
            let lt = compare_lt(ir.modifier, &a.inst, &b.inst);
            ExecResult { action: ExecAction::Advance(if lt { skip_pc } else { next_pc }), wrote_addr: None }
        }
    }
}

fn apply_mov(m: Modifier, src: &Instruction, dst: &mut Instruction) {
    match m {
        Modifier::A => dst.a_val = src.a_val,
        Modifier::B => dst.b_val = src.b_val,
        Modifier::Ab => dst.b_val = src.a_val,
        Modifier::Ba => dst.a_val = src.b_val,
        Modifier::F => { dst.a_val = src.a_val; dst.b_val = src.b_val; }
        Modifier::X => { dst.b_val = src.a_val; dst.a_val = src.b_val; }
        Modifier::I => *dst = *src,
    }
}

fn op_val(op: Opcode, lhs: u32, rhs: u32, n: u32) -> Option<u32> {
    match op {
        Opcode::Add => Some(add_mod(lhs, rhs, n)),
        Opcode::Sub => Some(sub_mod(lhs, rhs, n)),
        Opcode::Mul => Some(mul_mod(lhs, rhs, n)),
        Opcode::Div => if rhs == 0 { None } else { Some(lhs / rhs) },
        Opcode::Mod => if rhs == 0 { None } else { Some(lhs % rhs) },
        _ => unreachable!(),
    }
}

fn apply_arith(op: Opcode, m: Modifier, a: &Instruction, b: &Instruction, dst: &mut Instruction, n: u32) -> bool {
    match m {
        Modifier::A => match op_val(op, b.a_val, a.a_val, n) {
            Some(v) => { dst.a_val = v; true }
            None => false,
        },
        Modifier::B => match op_val(op, b.b_val, a.b_val, n) {
            Some(v) => { dst.b_val = v; true }
            None => false,
        },
        Modifier::Ab => match op_val(op, b.b_val, a.a_val, n) {
            Some(v) => { dst.b_val = v; true }
            None => false,
        },
        Modifier::Ba => match op_val(op, b.a_val, a.b_val, n) {
            Some(v) => { dst.a_val = v; true }
            None => false,
        },
        Modifier::F | Modifier::I => {
            let va = op_val(op, b.a_val, a.a_val, n);
            let vb = op_val(op, b.b_val, a.b_val, n);
            if let Some(v) = va { dst.a_val = v; }
            if let Some(v) = vb { dst.b_val = v; }
            va.is_some() && vb.is_some()
        }
        Modifier::X => {
            let va = op_val(op, b.a_val, a.b_val, n);
            let vb = op_val(op, b.b_val, a.a_val, n);
            if let Some(v) = va { dst.a_val = v; }
            if let Some(v) = vb { dst.b_val = v; }
            va.is_some() && vb.is_some()
        }
    }
}

fn compare_eq(m: Modifier, a: &Instruction, b: &Instruction) -> bool {
    match m {
        Modifier::A => a.a_val == b.a_val,
        Modifier::B => a.b_val == b.b_val,
        Modifier::Ab => a.a_val == b.b_val,
        Modifier::Ba => a.b_val == b.a_val,
        Modifier::F => a.a_val == b.a_val && a.b_val == b.b_val,
        Modifier::X => a.a_val == b.b_val && a.b_val == b.a_val,
        Modifier::I => a == b,
    }
}

fn compare_lt(m: Modifier, a: &Instruction, b: &Instruction) -> bool {
    match m {
        Modifier::A => a.a_val < b.a_val,
        Modifier::B => a.b_val < b.b_val,
        Modifier::Ab => a.a_val < b.b_val,
        Modifier::Ba => a.b_val < b.a_val,
        Modifier::F | Modifier::I => a.a_val < b.a_val && a.b_val < b.b_val,
        Modifier::X => a.a_val < b.b_val && a.b_val < b.a_val,
    }
}
