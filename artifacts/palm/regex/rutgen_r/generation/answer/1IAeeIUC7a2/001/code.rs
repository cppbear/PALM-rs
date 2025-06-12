// Answer 0

#[derive(Debug)]
enum MaybeInst {
    Split,
    Split2(InstPtr),
}

#[derive(Debug)]
struct InstPtr;

fn half_fill_split_goto2(inst: &mut MaybeInst, goto2: InstPtr) {
    let half_filled = match *inst {
        MaybeInst::Split => goto2,
        _ => unreachable!("must be called on Split instruction, instead it was called on: {:?}", inst),
    };
    *inst = MaybeInst::Split2(half_filled);
}

#[test]
#[should_panic(expected = "must be called on Split instruction")]
fn test_half_fill_split_goto2_with_non_split() {
    let mut inst = MaybeInst::Split2(InstPtr);
    let goto2 = InstPtr;
    half_fill_split_goto2(&mut inst, goto2);
}

#[test]
fn test_half_fill_split_goto2_with_split() {
    let mut inst = MaybeInst::Split;
    let goto2 = InstPtr;
    half_fill_split_goto2(&mut inst, goto2);
    match inst {
        MaybeInst::Split2(_) => (),
        _ => panic!("Expected MaybeInst::Split2 but got: {:?}", inst),
    }
}

