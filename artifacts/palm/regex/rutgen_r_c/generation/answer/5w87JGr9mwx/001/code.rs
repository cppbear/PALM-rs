// Answer 0

#[test]
#[should_panic]
fn test_fill_split_non_split() {
    struct DummyInstPtr;

    let mut instruction = MaybeInst::Compiled(Inst::Match(0));
    let goto1 = DummyInstPtr;
    let goto2 = DummyInstPtr;

    instruction.fill_split(goto1, goto2);
}

#[test]
fn test_fill_split_success() {
    struct DummyInstPtr;

    let mut instruction = MaybeInst::Split;
    let goto1 = DummyInstPtr;
    let goto2 = DummyInstPtr;

    instruction.fill_split(goto1, goto2);

    if let MaybeInst::Compiled(Inst::Split(inst_split)) = instruction {
        assert_eq!(inst_split.goto1, goto1);
        assert_eq!(inst_split.goto2, goto2);
    } else {
        panic!("Expected instruction to be Compiled with Split variant");
    }
}

