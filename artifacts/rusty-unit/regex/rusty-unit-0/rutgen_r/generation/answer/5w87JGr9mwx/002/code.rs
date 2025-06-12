// Answer 0

#[test]
fn test_fill_split_with_valid_split() {
    struct InstSplit {
        goto1: InstPtr,
        goto2: InstPtr,
    }

    enum MaybeInst {
        Split,
        Compiled(Inst),
    }

    struct Inst {
        split: InstSplit,
    }

    type InstPtr = *const Inst;

    // Setup an initial state to ensure *self matches MaybeInst::Split.
    let mut instruction = MaybeInst::Split;

    let goto1: InstPtr = std::ptr::null();
    let goto2: InstPtr = std::ptr::null();

    // Call the method under test
    if let MaybeInst::Split = instruction {
        instruction.fill_split(goto1, goto2);
    }

    // Verify that the state has changed to MaybeInst::Compiled with the correct Inst.
    if let MaybeInst::Compiled(inst) = instruction {
        assert_eq!(inst.split.goto1, goto1);
        assert_eq!(inst.split.goto2, goto2);
    } else {
        panic!("Expected instruction to be compiled, but it wasn't.");
    }
}

#[test]
#[should_panic(expected = "must be called on Split instruction")]
fn test_fill_split_should_panic_on_non_split() {
    struct InstSplit {
        goto1: InstPtr,
        goto2: InstPtr,
    }

    enum MaybeInst {
        Split,
        Compiled(Inst),
    }

    struct Inst {
        split: InstSplit,
    }

    type InstPtr = *const Inst;

    // Setup an initial state where the instruction is not MaybeInst::Split.
    let mut instruction = MaybeInst::Compiled(Inst {
        split: InstSplit { goto1: std::ptr::null(), goto2: std::ptr::null() },
    });

    let goto1: InstPtr = std::ptr::null();
    let goto2: InstPtr = std::ptr::null();

    // Call the method under test, expecting it to panic.
    instruction.fill_split(goto1, goto2);
}

