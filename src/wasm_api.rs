use std::cell::RefCell;
use crate::disasm::disassemble;
use crate::parser::assemble;
use crate::presets::PRESETS;
use crate::types::{MarsConfig, Opcode, WarriorProgram};
use crate::vm::{Mars, RoundStatus};

struct EngineState {
    config: MarsConfig,
    mars: Mars,
    programs: Vec<Option<WarriorProgram>>,
    render_buf: Vec<u32>,
    text_buf: Vec<u8>,
    stats_buf: Vec<u32>,
}

impl EngineState {
    fn new() -> Self {
        let config = MarsConfig::default();
        let n = config.core_size as usize;
        Self {
            config,
            mars: Mars::new(config),
            programs: vec![None, None, None, None],
            render_buf: vec![0; n],
            text_buf: Vec::new(),
            stats_buf: vec![0; 32],
        }
    }
}

thread_local! {
    static ENGINE: RefCell<EngineState> = RefCell::new(EngineState::new());
}

#[no_mangle]
pub extern "C" fn alloc_buf(size: usize) -> *mut u8 {
    let mut buf = vec![0u8; size];
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn free_buf(ptr: *mut u8, size: usize) {
    if !ptr.is_null() && size > 0 { unsafe { let _ = Vec::from_raw_parts(ptr, size, size); } }
}

#[no_mangle]
pub extern "C" fn init_arena(core_size: u32, max_cycles: u32, max_processes: u32, min_distance: u32) {
    ENGINE.with(|cell| {
        let mut st = cell.borrow_mut();
        let cfg = MarsConfig { core_size: core_size.max(64), max_cycles: max_cycles.max(10), max_processes: max_processes.max(1), max_length: 200, min_distance };
        st.config = cfg;
        st.mars = Mars::new(cfg);
        st.render_buf.resize(cfg.core_size as usize, 0);
        st.programs = vec![None, None, None, None];
    });
}

#[no_mangle]
pub extern "C" fn load_warrior(slot: usize, ptr: *const u8, len: usize) -> i32 {
    if slot >= 4 || ptr.is_null() { return -1; }
    let Ok(src) = std::str::from_utf8(unsafe { std::slice::from_raw_parts(ptr, len) }) else { return -2; };
    ENGINE.with(|cell| {
        let mut st = cell.borrow_mut();
        match assemble(src, &st.config) {
            Ok(prog) => {
                st.text_buf = format!("OK|{}|{}|{}", prog.name, prog.author, prog.instructions.len()).into_bytes();
                st.programs[slot] = Some(prog);
                0
            }
            Err(err) => {
                st.text_buf = format!("Line {}: {}", err.line, err.message).into_bytes();
                st.programs[slot] = None;
                err.line as i32
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn clear_warrior(slot: usize) {
    ENGINE.with(|cell| { let mut st = cell.borrow_mut(); if slot < st.programs.len() { st.programs[slot] = None; } });
}

#[no_mangle]
pub extern "C" fn start_round(seed: u32, start_offset: u32) -> i32 {
    ENGINE.with(|cell| {
        let mut st = cell.borrow_mut();
        let active: Vec<WarriorProgram> = st.programs.iter().filter_map(|p| p.clone()).collect();
        if active.is_empty() { return -1; }
        st.mars.load_warriors(&active, seed as u64 + 1, start_offset as usize);
        active.len() as i32
    })
}

#[no_mangle]
pub extern "C" fn step_cycles(count: u32) -> i32 {
    ENGINE.with(|cell| {
        let mut st = cell.borrow_mut();
        for _ in 0..count { if st.mars.step_cycle() != RoundStatus::Running { break; } }
        st.mars.status as i32
    })
}

#[no_mangle]
pub extern "C" fn update_render_buf() -> *const u32 {
    ENGINE.with(|cell| {
        let mut st = cell.borrow_mut();
        let EngineState { mars, render_buf, .. } = &mut *st;
        let n = mars.core.len();
        for i in 0..n {
            let inst = &mars.core[i];
            let owner = mars.owners[i] as u32;
            let is_dat = if inst.opcode == Opcode::Dat { 1 } else { 0 };
            let non_zero = if inst.a_val != 0 || inst.b_val != 0 || inst.opcode != Opcode::Dat { 1 } else { 0 };
            render_buf[i] = owner | (is_dat << 8) | (non_zero << 9);
        }
        for &(addr, wid) in &mars.recent_writes {
            if (addr as usize) < n {
                render_buf[addr as usize] |= (wid as u32) << 16 | (1 << 20);
            }
        }
        for &(addr, wid) in &mars.recent_execs {
            if (addr as usize) < n {
                render_buf[addr as usize] |= (wid as u32) << 24 | (1 << 28);
            }
        }
        render_buf.as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn update_stats_buf() -> *const u32 {
    ENGINE.with(|cell| {
        let mut st = cell.borrow_mut();
        let EngineState { mars, stats_buf, .. } = &mut *st;
        stats_buf.fill(0);
        stats_buf[0] = mars.cycle;
        stats_buf[1] = mars.status as u32;
        stats_buf[2] = mars.winner_id.unwrap_or(0) as u32;
        stats_buf[3] = mars.warriors.len() as u32;
        for (i, w) in mars.warriors.iter().take(4).enumerate() {
            let base = 4 + i * 4;
            stats_buf[base] = if w.alive { 1 } else { 0 };
            stats_buf[base + 1] = w.queue.len() as u32;
            stats_buf[base + 2] = w.writes;
            stats_buf[base + 3] = w.last_pc;
        }
        stats_buf.as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn inspect_cell(addr: u32) -> *const u8 {
    ENGINE.with(|cell| {
        let mut st = cell.borrow_mut();
        let idx = (addr as usize) % st.mars.core.len();
        let dis = disassemble(&st.mars.core[idx], st.config.core_size);
        let owner = st.mars.owners[idx];
        st.text_buf = format!("{:04}|W{}|{}", idx, owner, dis).into_bytes();
        st.text_buf.as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn get_preset_count() -> usize { PRESETS.len() }

#[no_mangle]
pub extern "C" fn get_preset_info(idx: usize) -> *const u8 {
    ENGINE.with(|cell| {
        let mut st = cell.borrow_mut();
        st.text_buf = match PRESETS.get(idx) {
            Some(p) => format!("{}|{}|{}|{}|{}", p.id, p.name, p.author, p.category, p.code).into_bytes(),
            None => Vec::new(),
        };
        st.text_buf.as_ptr()
    })
}

#[no_mangle]
pub extern "C" fn get_text_ptr() -> *const u8 { ENGINE.with(|cell| cell.borrow().text_buf.as_ptr()) }

#[no_mangle]
pub extern "C" fn get_text_len() -> usize { ENGINE.with(|cell| cell.borrow().text_buf.len()) }
