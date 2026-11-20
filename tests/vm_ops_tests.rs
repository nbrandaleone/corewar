use corewar::parser::assemble;
use corewar::types::{MarsConfig, Opcode};
use corewar::vm::{Mars, RoundStatus};

fn run_single(src: &str, cycles: u32) -> Mars {
    let cfg = MarsConfig {
        core_size: 100,
        max_cycles: 1000,
        max_processes: 100,
        max_length: 50,
        min_distance: 10,
    };
    let prog = assemble(src, &cfg).unwrap();
    let mut mars = Mars::new(cfg);
    mars.load_warriors(&[prog], 42, 0);
    for _ in 0..cycles {
        if mars.step_cycle() != RoundStatus::Running {
            break;
        }
    }
    mars
}

#[test]
fn test_imp_moves_forward() {
    let mars = run_single("MOV 0, 1", 3);
    assert_eq!(mars.core[0].opcode, Opcode::Mov);
    assert_eq!(mars.core[1].opcode, Opcode::Mov);
    assert_eq!(mars.core[2].opcode, Opcode::Mov);
    assert_eq!(mars.core[3].opcode, Opcode::Mov);
    assert_eq!(mars.core[4].opcode, Opcode::Dat);
    assert_eq!(mars.warriors[0].queue.front().copied(), Some(3));
}

#[test]
fn test_predec_and_postinc_modes() {
    let src = "\
        MOV.I   }ptr,   >dst
        JMP     10
ptr     DAT     #1,     #0
dst     DAT     #0,     #5";
    let mars = run_single(src, 1);
    assert_eq!(mars.core[2].a_val, 2);
    assert_eq!(mars.core[3].b_val, 6);
    assert_eq!(mars.core[8].opcode, Opcode::Dat);
    assert_eq!(mars.core[8].b_val, 5);
}

#[test]
fn test_arithmetic_and_div_by_zero_kills_process() {
    let src = "\
        ADD.AB  #5,     3
        SUB.AB  #2,     2
        MUL.AB  #3,     1
        DAT     #0,     #10";
    let mars = run_single(src, 3);
    assert_eq!(mars.core[3].b_val, 39);

    let div0 = run_single("DIV.AB #0, 1\nDAT #0, #5", 1);
    assert!(!div0.warriors[0].alive);
    assert_eq!(div0.status, RoundStatus::Tie);
}

#[test]
fn test_spl_and_djn_loop() {
    let src = "\
        SPL     target
        JMP     0
target  MOV.AB  #42,    2
        DJN     0,      #1
        DAT     #0,     #0";
    let mars = run_single(src, 4);
    assert_eq!(mars.warriors[0].queue.len(), 2);
    assert_eq!(mars.core[4].b_val, 42);
}

#[test]
fn test_seq_sne_slt_skip() {
    let src = "\
        SEQ.I   3,      4
        DAT     #0,     #0
        MOV.AB  #7,     2
        DAT     #1,     #1
        DAT     #1,     #1";
    let mars = run_single(src, 2);
    assert!(mars.warriors[0].alive);
    assert_eq!(mars.core[4].b_val, 7);
}
