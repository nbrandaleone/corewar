use crate::mars_math::{add_mod, dec_mod, inc_mod};
use crate::types::{Instruction, Mode};

pub struct EvalResult {
    pub addr: u32,
    pub inst: Instruction,
    pub side_write: Option<u32>,
}

pub fn eval_operand(
    core: &[Instruction],
    pc: u32,
    ir: &Instruction,
    mode: Mode,
    val: u32,
) -> EvalResult {
    let n = core.len() as u32;
    match mode {
        Mode::Immediate => EvalResult {
            addr: pc,
            inst: *ir,
            side_write: None,
        },
        Mode::Direct => {
            let addr = add_mod(pc, val, n);
            EvalResult {
                addr,
                inst: core[addr as usize],
                side_write: None,
            }
        }
        Mode::AIndirect => {
            let ptr = add_mod(pc, val, n);
            let addr = add_mod(ptr, core[ptr as usize].a_val, n);
            EvalResult {
                addr,
                inst: core[addr as usize],
                side_write: None,
            }
        }
        Mode::BIndirect => {
            let ptr = add_mod(pc, val, n);
            let addr = add_mod(ptr, core[ptr as usize].b_val, n);
            EvalResult {
                addr,
                inst: core[addr as usize],
                side_write: None,
            }
        }
        _ => unreachable!(),
    }
}

pub fn eval_operand_mut(
    core: &mut [Instruction],
    pc: u32,
    ir: &Instruction,
    mode: Mode,
    val: u32,
) -> EvalResult {
    let n = core.len() as u32;
    match mode {
        Mode::Immediate | Mode::Direct | Mode::AIndirect | Mode::BIndirect => {
            eval_operand(core, pc, ir, mode, val)
        }
        Mode::APreDec => {
            let ptr = add_mod(pc, val, n);
            let next_a = dec_mod(core[ptr as usize].a_val, n);
            core[ptr as usize].a_val = next_a;
            let addr = add_mod(ptr, next_a, n);
            EvalResult {
                addr,
                inst: core[addr as usize],
                side_write: Some(ptr),
            }
        }
        Mode::BPreDec => {
            let ptr = add_mod(pc, val, n);
            let next_b = dec_mod(core[ptr as usize].b_val, n);
            core[ptr as usize].b_val = next_b;
            let addr = add_mod(ptr, next_b, n);
            EvalResult {
                addr,
                inst: core[addr as usize],
                side_write: Some(ptr),
            }
        }
        Mode::APostInc => {
            let ptr = add_mod(pc, val, n);
            let offset = core[ptr as usize].a_val;
            let addr = add_mod(ptr, offset, n);
            let inst = core[addr as usize];
            core[ptr as usize].a_val = inc_mod(offset, n);
            EvalResult {
                addr,
                inst,
                side_write: Some(ptr),
            }
        }
        Mode::BPostInc => {
            let ptr = add_mod(pc, val, n);
            let offset = core[ptr as usize].b_val;
            let addr = add_mod(ptr, offset, n);
            let inst = core[addr as usize];
            core[ptr as usize].b_val = inc_mod(offset, n);
            EvalResult {
                addr,
                inst,
                side_write: Some(ptr),
            }
        }
    }
}
