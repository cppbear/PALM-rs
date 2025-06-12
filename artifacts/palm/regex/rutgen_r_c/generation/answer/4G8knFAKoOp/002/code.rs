// Answer 0

#[test]
fn test_skip_with_consecutive_save_instructions() {
    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 2, slot: 0 }),
            Inst::Save(InstSave { goto: 4, slot: 1 }),
            Inst::Match(0), // a match instruction
            Inst::Save(InstSave { goto: 6, slot: 2 }),
            Inst::Match(1), // a match instruction
            Inst::Save(InstSave { goto: 0, slot: 0 }) // going back to an earlier instruction
        ],
        ..Program::new()
    };
    
    let pc = program.skip(0);
    assert_eq!(pc, 2); // should skip to Match(0)
}

#[test]
fn test_skip_with_single_save_instruction() {
    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 1, slot: 0 }),
            Inst::Match(0) // should return this instruction
        ],
        ..Program::new()
    };
    
    let pc = program.skip(0);
    assert_eq!(pc, 1); // should skip to Match(0)
}

#[test]
fn test_skip_reaches_match_immediately() {
    let program = Program {
        insts: vec![
            Inst::Match(0), // should return immediately
        ],
        ..Program::new()
    };
    
    let pc = program.skip(0);
    assert_eq!(pc, 0); // should not skip
}

#[test]
fn test_skip_with_no_save_instructions() {
    let program = Program {
        insts: vec![
            Inst::Match(0), // first instruction
        ],
        ..Program::new()
    };
    
    let pc = program.skip(0);
    assert_eq!(pc, 0); // should return the same instruction
}

#[test]
#[should_panic]
fn test_skip_with_invalid_pc() {
    let program = Program {
        insts: vec![
            Inst::Save(InstSave { goto: 1, slot: 0 }),
        ],
        ..Program::new()
    };
    
    program.skip(2); // this should panic: invalid index
}

