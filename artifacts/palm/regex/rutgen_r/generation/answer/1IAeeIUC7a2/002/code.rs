// Answer 0

#[derive(Debug)]
enum MaybeInst {
    Split,
    Split2(InstPtr),
}

type InstPtr = u32;

fn half_fill_split_goto2(inst: &mut MaybeInst, goto2: InstPtr) {
    let half_filled = match *inst {
        MaybeInst::Split => goto2,
        _ => unreachable!("must be called on Split instruction, \
                           instead it was called on: {:?}", inst),
    };
    *inst = MaybeInst::Split2(half_filled);
}

#[test]
fn test_half_fill_split_goto2_with_valid_state() {
    let mut inst = MaybeInst::Split;
    let goto2: InstPtr = 5;

    half_fill_split_goto2(&mut inst, goto2);

    if let MaybeInst::Split2(value) = inst {
        assert_eq!(value, goto2);
    } else {
        panic!("Expected Split2 variant but got a different variant.");
    }
}

#[test]
#[should_panic(expected = "must be called on Split instruction")]
fn test_half_fill_split_goto2_with_invalid_state() {
    let mut inst = MaybeInst::Split2(3); // Invalid initial state
    let goto2: InstPtr = 7;

    half_fill_split_goto2(&mut inst, goto2);
}

