// Answer 0

#[test]
fn test_skip_no_op_instruction() {
    let mut program = Program::new();
    program.insts.push(Inst::Save(InstSave { goto: 1, slot: 0 }));
    program.insts.push(Inst::Match(0));

    assert_eq!(program.skip(0), 1);
}

#[test]
fn test_skip_multiple_no_op_instructions() {
    let mut program = Program::new();
    program.insts.push(Inst::Save(InstSave { goto: 2, slot: 0 }));
    program.insts.push(Inst::Save(InstSave { goto: 3, slot: 1 }));
    program.insts.push(Inst::Match(0));

    assert_eq!(program.skip(0), 3);
}

#[test]
fn test_skip_direct_match() {
    let mut program = Program::new();
    program.insts.push(Inst::Match(0));

    assert_eq!(program.skip(0), 0);
}

#[test]
fn test_skip_empty_instructions() {
    let mut program = Program::new();
    program.insts.push(Inst::EmptyLook(InstEmptyLook {})); // Assuming InstEmptyLook exists
    program.insts.push(Inst::Match(0));

    assert_eq!(program.skip(0), 0);
}

#[test]
fn test_skip_with_other_instructions() {
    let mut program = Program::new();
    program.insts.push(Inst::Save(InstSave { goto: 2, slot: 0 }));
    program.insts.push(Inst::Char(InstChar {})); // Assuming InstChar exists
    program.insts.push(Inst::Match(0));

    assert_eq!(program.skip(0), 1);
}

