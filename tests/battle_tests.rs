use corewar::parser::assemble;
use corewar::presets::PRESETS;
use corewar::types::MarsConfig;
use corewar::vm::{Mars, RoundStatus};

#[test]
fn test_all_presets_assemble_cleanly() {
    let cfg = MarsConfig::default();
    for preset in PRESETS {
        let res = assemble(preset.code, &cfg);
        assert!(res.is_ok(), "Preset '{}' failed to assemble: {:?}", preset.name, res.err());
    }
}

#[test]
fn test_dwarf_vs_sitting_duck() {
    let cfg = MarsConfig::default();
    let dwarf = assemble(PRESETS[0].code, &cfg).unwrap();
    let duck = assemble("NOP 0, 0\nNOP 0, 0\nNOP 0, 0\nJMP -3", &cfg).unwrap();
    let mut mars = Mars::new(cfg);
    mars.load_warriors(&[dwarf, duck], 12345, 0);
    while mars.step_cycle() == RoundStatus::Running {}
    assert_eq!(mars.status, RoundStatus::Winner);
    assert_eq!(mars.winner_id, Some(1));
}

#[test]
fn test_mice_vs_dwarf_battle() {
    let cfg = MarsConfig::default();
    let mice = assemble(PRESETS[2].code, &cfg).unwrap();
    let dwarf = assemble(PRESETS[0].code, &cfg).unwrap();
    let mut mars = Mars::new(cfg);
    mars.load_warriors(&[mice, dwarf], 99, 0);
    while mars.step_cycle() == RoundStatus::Running {}
    assert!(mars.cycle > 100);
    assert_ne!(mars.status, RoundStatus::Running);
}
