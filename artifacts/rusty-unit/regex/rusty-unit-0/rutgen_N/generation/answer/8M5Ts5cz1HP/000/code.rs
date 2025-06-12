// Answer 0

#[derive(Debug)]
enum MaybeInst {
    Split,
    Split1(InstPtr),
}

type InstPtr = usize;

fn half_fill_split_goto1(maybe_inst: &mut MaybeInst, goto1: InstPtr) {
    let half_filled = match *maybe_inst {
        MaybeInst::Split => goto1,
        _ => unreachable!("must be called on Split instruction, instead it was called on: {:?}", maybe_inst),
    };
    *maybe_inst = MaybeInst::Split1(half_filled);
}

#[test]
fn test_half_fill_split_goto1_on_split() {
    let mut maybe_inst = MaybeInst::Split;
    let goto1: InstPtr = 42;
    half_fill_split_goto1(&mut maybe_inst, goto1);
    match maybe_inst {
        MaybeInst::Split1(value) => assert_eq!(value, goto1),
        _ => panic!("Expected MaybeInst::Split1, got: {:?}", maybe_inst),
    }
}

#[test]
#[should_panic(expected = "must be called on Split instruction")]
fn test_half_fill_split_goto1_on_invalid_variant() {
    let mut maybe_inst = MaybeInst::Split1(0);
    let goto1: InstPtr = 42;
    half_fill_split_goto1(&mut maybe_inst, goto1);
}

