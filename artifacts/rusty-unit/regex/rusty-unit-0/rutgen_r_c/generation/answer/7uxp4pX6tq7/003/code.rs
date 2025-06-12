// Answer 0

#[test]
fn test_fill_split1() {
    use prog::{InstPtr, InstSave};

    // Define local structures for test
    let goto1: InstPtr = 1; // Use a simple instantiated pointer
    let goto2: InstPtr = 2; // Use a simple instantiated pointer
    let mut maybe_inst = MaybeInst::Split1(goto1);

    // Call the fill method with goto
    maybe_inst.fill(goto2);

    // Check the state of maybe_inst after fill
    if let MaybeInst::Compiled(inst) = maybe_inst {
        match inst {
            Inst::Split(inst_split) => {
                assert_eq!(inst_split.goto1, goto1);
                assert_eq!(inst_split.goto2, goto2);
            },
            _ => panic!("Expected Inst::Split but found another variant."),
        }
    } else {
        panic!("Expected MaybeInst::Compiled but found another variant.");
    }
}

#[test]
fn test_fill_split2() {
    use prog::{InstPtr, InstSave};

    // Define local structures for test
    let goto1: InstPtr = 1; // Use a simple instantiated pointer
    let goto2: InstPtr = 2; // Use a simple instantiated pointer
    let mut maybe_inst = MaybeInst::Split2(goto2);

    // Call the fill method with goto
    maybe_inst.fill(goto1);

    // Check the state of maybe_inst after fill
    if let MaybeInst::Compiled(inst) = maybe_inst {
        match inst {
            Inst::Split(inst_split) => {
                assert_eq!(inst_split.goto1, goto1);
                assert_eq!(inst_split.goto2, goto2);
            },
            _ => panic!("Expected Inst::Split but found another variant."),
        }
    } else {
        panic!("Expected MaybeInst::Compiled but found another variant.");
    }
}

#[test]
#[should_panic]
fn test_fill_uncompiled() {
    let mut maybe_inst = MaybeInst::Uncompiled(InstHole::Char { c: 'a' });
    let goto: InstPtr = 1; // Use a simple instantiated pointer

    // Call the fill method which should panic
    maybe_inst.fill(goto);
}

