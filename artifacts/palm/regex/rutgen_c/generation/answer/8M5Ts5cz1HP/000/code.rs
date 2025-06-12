// Answer 0

#[test]
fn test_half_fill_split_goto1() {
    struct MockInstPtr;
    let goto1 = MockInstPtr;

    let mut maybe_inst = MaybeInst::Split;

    maybe_inst.half_fill_split_goto1(goto1);

    match maybe_inst {
        MaybeInst::Split1(ref ptr) => {
            assert_eq!(ptr, &goto1);
        },
        _ => {
            panic!("Expected MaybeInst::Split1 variant");
        }
    }
}

#[test]
#[should_panic(expected = "must be called on Split instruction")]
fn test_half_fill_split_goto1_unreachable() {
    struct MockInstPtr;
    let goto1 = MockInstPtr;

    let mut maybe_inst = MaybeInst::Compiled(Inst::Match(0));

    maybe_inst.half_fill_split_goto1(goto1);
}

