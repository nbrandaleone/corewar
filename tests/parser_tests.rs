use corewar::parser::assemble;
use corewar::types::{MarsConfig, Mode, Modifier, Opcode};

#[test]
fn test_assemble_imp() {
    let cfg = MarsConfig::default();
    let prog = assemble(";name Imp\n;author Dewdney\nMOV 0, 1\nEND", &cfg).unwrap();
    assert_eq!(prog.name, "Imp");
    assert_eq!(prog.author, "Dewdney");
    assert_eq!(prog.instructions.len(), 1);
    let i = prog.instructions[0];
    assert_eq!(i.opcode, Opcode::Mov);
    assert_eq!(i.modifier, Modifier::I);
    assert_eq!(i.a_mode, Mode::Direct);
    assert_eq!(i.a_val, 0);
    assert_eq!(i.b_mode, Mode::Direct);
    assert_eq!(i.b_val, 1);
}

#[test]
fn test_assemble_dwarf_with_labels_and_org() {
    let cfg = MarsConfig::default();
    let src = "\
;redcode-94
;name Dwarf
        ORG     start
bomb    DAT     #0,     #0
start   ADD.AB  #4,     bomb
        MOV.AB  #0,     @bomb
        JMP     start
        END";
    let prog = assemble(src, &cfg).unwrap();
    assert_eq!(prog.org, 1);
    assert_eq!(prog.instructions.len(), 4);
    assert_eq!(prog.instructions[1].opcode, Opcode::Add);
    assert_eq!(prog.instructions[1].modifier, Modifier::Ab);
    assert_eq!(prog.instructions[1].a_val, 4);
    assert_eq!(prog.instructions[1].b_val, 7999);
    assert_eq!(prog.instructions[3].opcode, Opcode::Jmp);
    assert_eq!(prog.instructions[3].a_val, 7998);
}

#[test]
fn test_assemble_equ_and_expressions() {
    let cfg = MarsConfig::default();
    let src = "\
step    EQU     (2+3)*4
        ADD     #step,  $1
        DAT     #-1";
    let prog = assemble(src, &cfg).unwrap();
    assert_eq!(prog.instructions[0].a_val, 20);
    assert_eq!(prog.instructions[1].opcode, Opcode::Dat);
    assert_eq!(prog.instructions[1].b_val, 7999);
}

#[test]
fn test_default_modifiers() {
    let cfg = MarsConfig::default();
    let src = "\
        MOV     #1,     2
        MOV     1,      #2
        MOV     1,      2
        ADD     #1,     2
        ADD     1,      2
        SLT     #1,     2
        SLT     1,      2";
    let prog = assemble(src, &cfg).unwrap();
    assert_eq!(prog.instructions[0].modifier, Modifier::Ab);
    assert_eq!(prog.instructions[1].modifier, Modifier::B);
    assert_eq!(prog.instructions[2].modifier, Modifier::I);
    assert_eq!(prog.instructions[3].modifier, Modifier::Ab);
    assert_eq!(prog.instructions[4].modifier, Modifier::F);
    assert_eq!(prog.instructions[5].modifier, Modifier::Ab);
    assert_eq!(prog.instructions[6].modifier, Modifier::B);
}

#[test]
fn test_parse_errors() {
    let cfg = MarsConfig::default();
    assert!(assemble("", &cfg).is_err());
    assert!(assemble("FOO 0, 1", &cfg).is_err());
    assert!(assemble("MOV.Z 0, 1", &cfg).is_err());
    assert!(assemble("JMP unknown_label", &cfg).is_err());
}
