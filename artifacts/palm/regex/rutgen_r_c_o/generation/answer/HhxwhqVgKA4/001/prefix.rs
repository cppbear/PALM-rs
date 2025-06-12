// Answer 0

#[test]
fn test_leads_to_match_multiple_matches() {
    let mut program = Program::new();
    program.matches.push(1);
    program.matches.push(2);
    program.insts.push(Inst::Match(0));
    program.insts.push(Inst::Char(InstChar { c: 'a' }));

    let pc = 0;
    program.leads_to_match(pc);
}

#[test]
fn test_leads_to_match_only_two_matches() {
    let mut program = Program::new();
    program.matches.push(1);
    program.matches.push(3);
    program.insts.push(Inst::Match(1));
    program.insts.push(Inst::Save(InstSave { goto: 1 }));

    let pc = 1;
    program.leads_to_match(pc);
}

#[test]
fn test_leads_to_match_multiple_saves() {
    let mut program = Program::new();
    program.matches.push(1);
    program.matches.push(2);
    program.insts.push(Inst::Save(InstSave { goto: 2 }));
    program.insts.push(Inst::Char(InstChar { c: 'b' }));

    let pc = 0;
    program.leads_to_match(pc);
}

#[test]
fn test_leads_to_match_empty_instructions() {
    let mut program = Program::new();
    program.matches.push(1);
    program.matches.push(2);
    program.insts.push(Inst::EmptyLook(InstEmptyLook {}));

    let pc = 0;
    program.leads_to_match(pc);
}

#[test]
fn test_leads_to_match_with_non_match_inst() {
    let mut program = Program::new();
    program.matches.push(2);
    program.matches.push(3);
    program.insts.push(Inst::Save(InstSave { goto: 1 }));
    program.insts.push(Inst::Ranges(InstRanges {}));

    let pc = 0;
    program.leads_to_match(pc);
}

