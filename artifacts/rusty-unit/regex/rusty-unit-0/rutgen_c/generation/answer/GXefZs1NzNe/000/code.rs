// Answer 0

#[test]
fn test_push_split_hole_empty_insts() {
    let mut compiler = Compiler::new();
    let hole = compiler.push_split_hole();
    assert_eq!(hole, Hole::One(0));
    assert_eq!(compiler.insts.len(), 1);
    match &compiler.insts[0] {
        MaybeInst::Split => {}
        _ => panic!("Expected MaybeInst::Split"),
    }
}

#[test]
fn test_push_split_hole_non_empty_insts() {
    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Compiled(Inst::new())); // Add an instance
    let hole = compiler.push_split_hole();
    assert_eq!(hole, Hole::One(1));
    assert_eq!(compiler.insts.len(), 2);
    match &compiler.insts[1] {
        MaybeInst::Split => {}
        _ => panic!("Expected MaybeInst::Split"),
    }
}

