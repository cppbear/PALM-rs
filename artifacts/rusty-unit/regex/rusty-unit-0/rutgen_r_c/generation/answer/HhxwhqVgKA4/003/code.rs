// Answer 0

fn test_leads_to_match_single_match() {
    // Construct the necessary structure to test `leads_to_match`.
    let mut program = Program::new();
    
    // Set up a single match instruction
    program.matches.push(0); // Only one match
    program.insts.push(Inst::Match(0)); // Inst at index 0 is a Match instruction
    
    // Test with the program counter at 0, which should lead to a match
    assert_eq!(program.leads_to_match(0), true);
}

fn test_leads_to_match_single_match_after_skip() {
    // Construct the necessary structure to test `leads_to_match`.
    let mut program = Program::new();
    
    // Set up a single match instruction
    program.matches.push(0); // Only one match
    program.insts.push(Inst::Save(InstSave { goto: 1 })); // Inst at index 0 is a Save instruction
    program.insts.push(Inst::Match(0)); // Inst at index 1 is a Match instruction
    
    // Test with the program counter at 0, which should skip to index 1 and lead to a match
    assert_eq!(program.leads_to_match(0), true);
}

fn test_leads_to_match_fail_due_to_multiple_matches() {
    // Construct the necessary structure to test `leads_to_match`.
    let mut program = Program::new();
    
    // Set up multiple match instructions
    program.matches.push(0);
    program.matches.push(1); // Two matches, should return false
    program.insts.push(Inst::Match(0));
    
    // Test with the program counter at 0, which should return false due to multiple matches
    assert_eq!(program.leads_to_match(0), false);
}

