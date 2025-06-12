// Answer 0

#[test]
fn test_leads_to_match_single_match() {
    let mut program = Program::new();
    program.matches.push(0);  // Matches length is 1
    program.insts.push(Inst::Match(0));  // Adding a Match instruction
    program.insts.push(Inst::Save(InstSave { goto: 1 })); // Save instruction that points to Match
    let pc = 0;  // Valid pc within bounds
    program.leads_to_match(pc);
}

#[test]
fn test_leads_to_match_with_skip() {
    let mut program = Program::new();
    program.matches.push(0);  // Matches length is 1
    program.insts.push(Inst::Save(InstSave { goto: 1 })); // Save instruction
    program.insts.push(Inst::Match(0));  // Match instruction after Save
    let pc = 0;  // Valid pc within bounds
    program.leads_to_match(pc);
}

#[test]
fn test_leads_to_match_with_multiple_save() {
    let mut program = Program::new();
    program.matches.push(0);  // Matches length is 1
    program.insts.push(Inst::Save(InstSave { goto: 2 })); // First Save instruction
    program.insts.push(Inst::Save(InstSave { goto: 3 })); // Second Save instruction
    program.insts.push(Inst::Match(0));  // Match instruction
    let pc = 0;  // Valid pc within bounds
    program.leads_to_match(pc);
}

