// Answer 0

fn test_half_fill_split_goto1_valid() {
    #[derive(Debug)]
    enum MaybeInst {
        Split,
        Split1(InstPtr),
    }

    type InstPtr = usize;

    let mut inst = MaybeInst::Split; // *self matches MaybeInst::Split
    let goto1: InstPtr = 42; // A valid InstPtr to pass

    // Call the method under test
    half_fill_split_goto1(&mut inst, goto1);

    // Validate the result
    if let MaybeInst::Split1(ptr) = inst {
        assert_eq!(ptr, goto1);
    } else {
        panic!("Expected MaybeInst::Split1, but got: {:?}", inst);
    }
}

#[should_panic(expected = "must be called on Split instruction")]
fn test_half_fill_split_goto1_unreachable() {
    #[derive(Debug)]
    enum MaybeInst {
        Split,
        Split1(usize),
        Other,
    }

    type InstPtr = usize;

    let mut inst = MaybeInst::Other; // *self does not match MaybeInst::Split
    let goto1: InstPtr = 42;

    // This call should panic
    half_fill_split_goto1(&mut inst, goto1);
}

fn half_fill_split_goto1(inst: &mut MaybeInst, goto1: InstPtr) {
    let half_filled = match *inst {
        MaybeInst::Split => goto1,
        _ => unreachable!("must be called on Split instruction, \
                           instead it was called on: {:?}", inst),
    };
    *inst = MaybeInst::Split1(half_filled);
}

