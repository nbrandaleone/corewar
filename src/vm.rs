use std::collections::VecDeque;
use crate::mars_math::{add_mod, Rng};
use crate::types::{Instruction, MarsConfig, WarriorProgram};
use crate::vm_eval::eval_operand_mut;
use crate::vm_exec::{execute_inst, ExecAction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundStatus {
    Running = 0,
    Winner = 1,
    Tie = 2,
}

pub struct WarriorState {
    pub id: u8,
    pub start_addr: u32,
    pub queue: VecDeque<u32>,
    pub alive: bool,
    pub writes: u32,
    pub last_pc: u32,
}

pub struct Mars {
    pub config: MarsConfig,
    pub core: Vec<Instruction>,
    pub owners: Vec<u8>,
    pub warriors: Vec<WarriorState>,
    pub cycle: u32,
    pub turn_idx: usize,
    pub status: RoundStatus,
    pub winner_id: Option<u8>,
    pub recent_writes: Vec<(u32, u8)>,
    pub recent_execs: Vec<(u32, u8)>,
}

impl Mars {
    pub fn new(config: MarsConfig) -> Self {
        let len = config.core_size as usize;
        Self {
            config,
            core: vec![Instruction::default(); len],
            owners: vec![0; len],
            warriors: Vec::new(),
            cycle: 0,
            turn_idx: 0,
            status: RoundStatus::Running,
            winner_id: None,
            recent_writes: Vec::new(),
            recent_execs: Vec::new(),
        }
    }

    pub fn load_warriors(&mut self, programs: &[WarriorProgram], seed: u64, start_order_offset: usize) {
        let n = self.config.core_size;
        self.core.fill(Instruction::default());
        self.owners.fill(0);
        self.warriors.clear();
        self.cycle = 0;
        self.turn_idx = start_order_offset % programs.len().max(1);
        self.status = RoundStatus::Running;
        self.winner_id = None;
        self.recent_writes.clear();
        self.recent_execs.clear();

        let mut rng = Rng::new(seed);
        let placements = compute_placements(programs.len(), n, self.config.min_distance, &mut rng);

        for (i, prog) in programs.iter().enumerate() {
            let wid = (i + 1) as u8;
            let base = placements[i];
            for (offset, inst) in prog.instructions.iter().enumerate() {
                let addr = add_mod(base, offset as u32, n) as usize;
                self.core[addr] = *inst;
                self.owners[addr] = wid;
            }
            let entry = add_mod(base, prog.org as u32, n);
            let mut queue = VecDeque::new();
            queue.push_back(entry);
            self.warriors.push(WarriorState {
                id: wid,
                start_addr: base,
                queue,
                alive: true,
                writes: 0,
                last_pc: entry,
            });
        }
    }

    pub fn step_cycle(&mut self) -> RoundStatus {
        if self.status != RoundStatus::Running || self.warriors.is_empty() {
            return self.status;
        }
        self.recent_writes.clear();
        self.recent_execs.clear();

        let total_w = self.warriors.len();
        for _ in 0..total_w {
            let idx = self.turn_idx % total_w;
            self.turn_idx = (self.turn_idx + 1) % total_w;
            if !self.warriors[idx].alive {
                continue;
            }
            if let Some(pc) = self.warriors[idx].queue.pop_front() {
                let wid = self.warriors[idx].id;
                self.warriors[idx].last_pc = pc;
                self.recent_execs.push((pc, wid));
                let ir = self.core[pc as usize];
                let a = eval_operand_mut(&mut self.core, pc, &ir, ir.a_mode, ir.a_val);
                if let Some(w) = a.side_write {
                    self.owners[w as usize] = wid;
                    self.warriors[idx].writes += 1;
                    self.recent_writes.push((w, wid));
                }
                let b = eval_operand_mut(&mut self.core, pc, &ir, ir.b_mode, ir.b_val);
                if let Some(w) = b.side_write {
                    self.owners[w as usize] = wid;
                    self.warriors[idx].writes += 1;
                    self.recent_writes.push((w, wid));
                }
                let res = execute_inst(&mut self.core, pc, &ir, &a, &b);
                if let Some(w) = res.wrote_addr {
                    self.owners[w as usize] = wid;
                    self.warriors[idx].writes += 1;
                    self.recent_writes.push((w, wid));
                }
                match res.action {
                    ExecAction::Advance(npc) | ExecAction::Jump(npc) => {
                        self.warriors[idx].queue.push_back(npc);
                    }
                    ExecAction::Split(npc, target) => {
                        self.warriors[idx].queue.push_back(npc);
                        if (self.warriors[idx].queue.len() as u32) < self.config.max_processes {
                            self.warriors[idx].queue.push_back(target);
                        }
                    }
                    ExecAction::Die => {
                        if self.warriors[idx].queue.is_empty() {
                            self.warriors[idx].alive = false;
                        }
                    }
                }
            } else {
                self.warriors[idx].alive = false;
            }
            if self.check_end_condition() {
                return self.status;
            }
        }

        self.cycle += 1;
        if self.cycle >= self.config.max_cycles {
            self.status = RoundStatus::Tie;
        }
        self.status
    }

    fn check_end_condition(&mut self) -> bool {
        let alive_count = self.warriors.iter().filter(|w| w.alive).count();
        if self.warriors.len() == 1 {
            if alive_count == 0 {
                self.status = RoundStatus::Tie;
                return true;
            }
            return false;
        }
        if alive_count <= 1 {
            if let Some(w) = self.warriors.iter().find(|w| w.alive) {
                self.status = RoundStatus::Winner;
                self.winner_id = Some(w.id);
            } else {
                self.status = RoundStatus::Tie;
            }
            return true;
        }
        false
    }
}

fn compute_placements(count: usize, core_size: u32, min_dist: u32, rng: &mut Rng) -> Vec<u32> {
    let mut pos = Vec::with_capacity(count);
    if count == 0 { return pos; }
    pos.push(0);
    for _ in 1..count {
        let mut chosen = 0;
        for _ in 0..200 {
            let cand = rng.gen_range(core_size);
            if pos.iter().all(|&p| (cand + core_size - p) % core_size >= min_dist && (p + core_size - cand) % core_size >= min_dist) {
                chosen = cand;
                break;
            }
        }
        if chosen == 0 {
            chosen = ((pos.len() as u32) * (core_size / count as u32)) % core_size;
        }
        pos.push(chosen);
    }
    pos
}
