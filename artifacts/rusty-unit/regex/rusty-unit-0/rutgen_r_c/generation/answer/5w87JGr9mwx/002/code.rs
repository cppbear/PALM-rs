// Answer 0

#[test]
fn test_fill_split_with_valid_gotos() {
    // Arrange
    let goto1: InstPtr = InstPtr::default(); // assuming InstPtr has a default implementation
    let goto2: InstPtr = InstPtr::default(); // assuming InstPtr has a default implementation
    let mut maybe_inst = MaybeInst::Split;

    // Act
    maybe_inst.fill_split(goto1, goto2);

    // Assert
    if let MaybeInst::Compiled(Inst::Split(InstSplit { goto1: g1, goto2: g2 })) = maybe_inst {
        assert_eq!(g1, goto1);
        assert_eq!(g2, goto2);
    } else {
        panic!("Expected MaybeInst::Compiled with Inst::Split, but got: {:?}", maybe_inst);
    }
}

#[test]
#[should_panic(expected = "must be called on Split instruction, instead it was called on:")]
fn test_fill_split_with_non_split() {
    // Arrange
    let goto1: InstPtr = InstPtr::default(); // assuming InstPtr has a default implementation
    let goto2: InstPtr = InstPtr::default(); // assuming InstPtr has a default implementation
    let mut maybe_inst = MaybeInst::Compiled(Inst::Match(0)); // setting to a non-split variant

    // Act
    maybe_inst.fill_split(goto1, goto2);
}

