// Answer 0

#[test]
fn test_fill_split_success() {
    struct TestInstPtr;
    
    impl Copy for TestInstPtr {}
    impl Clone for TestInstPtr {
        fn clone(&self) -> Self {
            *self
        }
    }

    let mut maybe_inst = MaybeInst::Split;
    let goto1 = TestInstPtr;
    let goto2 = TestInstPtr;

    maybe_inst.fill_split(goto1, goto2);

    if let MaybeInst::Compiled(Inst::Split(inst_split)) = maybe_inst {
        assert_eq!(inst_split.goto1, goto1);
        assert_eq!(inst_split.goto2, goto2);
    } else {
        panic!("Expected MaybeInst::Compiled with Inst::Split");
    }
}

#[test]
#[should_panic(expected = "must be called on Split instruction")]
fn test_fill_split_unreachable() {
    let mut maybe_inst = MaybeInst::Compiled(Inst::Match(0));
    let goto1 = TestInstPtr;
    let goto2 = TestInstPtr;

    maybe_inst.fill_split(goto1, goto2);
}

