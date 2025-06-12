// Answer 0

#[test]
fn test_fill_split2() {
    struct InstPtrMock(usize);
    
    // Create a placeholder for the Inst and InstPtr
    let goto1 = InstPtrMock(1);
    let goto2 = InstPtrMock(2);
    
    // Create an instance of MaybeInst::Split2 with a mock InstPtr
    let mut instruction = MaybeInst::Split2(goto2);
    
    // Call the fill function with an InstPtr
    instruction.fill(goto1);
    
    // Compare the result, unwrap the instruction to ensure it is now Compiled
    if let MaybeInst::Compiled(inst) = instruction {
        if let Inst::Split(inst_split) = inst {
            assert_eq!(inst_split.goto1, goto1);
            assert_eq!(inst_split.goto2, goto2);
        } else {
            panic!("Expected an Inst::Split instruction");
        }
    } else {
        panic!("Expected an Compiled instruction");
    }
}

#[test]
#[should_panic(expected = "not all instructions were compiled!")]
fn test_fill_uncompiled_panic() {
    // Create a placeholder for the Inst and InstPtr
    let goto = InstPtrMock(1);
    
    // Create an instance of MaybeInst::Compiled to trigger panic
    let mut instruction = MaybeInst::Compiled(Inst::Match(0));
    
    // This should panic as the instruction is already compiled
    instruction.fill(goto);
}

#[test]
fn test_fill_split1() {
    struct InstPtrMock(usize);
    
    // Create a placeholder for the Inst and InstPtr
    let goto1 = InstPtrMock(1);
    let goto2 = InstPtrMock(2);
    
    // Create an instance of MaybeInst::Split1 with a mock InstPtr
    let mut instruction = MaybeInst::Split1(goto1);
    
    // Call the fill function with an InstPtr
    instruction.fill(goto2);
    
    // Compare the result, unwrap the instruction to ensure it is now Compiled
    if let MaybeInst::Compiled(inst) = instruction {
        if let Inst::Split(inst_split) = inst {
            assert_eq!(inst_split.goto1, goto1);
            assert_eq!(inst_split.goto2, goto2);
        } else {
            panic!("Expected an Inst::Split instruction");
        }
    } else {
        panic!("Expected an Compiled instruction");
    }
}

