// Answer 0

#[test]
fn test_fill_split_with_none_goto1() {
    let pc = 0;
    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Split);
    let hole = Hole::One(pc);
    let goto1 = None;
    let goto2 = Some(2); // Simulating a valid instruction pointer

    let result = compiler.fill_split(hole, goto1, goto2);
    assert_eq!(result, Hole::One(pc));
}

#[test]
fn test_fill_split_with_some_goto2() {
    let pc = 1;
    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Split);
    let hole = Hole::One(pc);
    let goto1 = Some(3); // Simulating a valid instruction pointer
    let goto2 = None;

    let result = compiler.fill_split(hole, goto1, goto2);
    assert_eq!(result, Hole::One(pc));
}

#[test]
#[should_panic(expected = "at least one of the split holes must be filled")]
fn test_fill_split_with_none_goto1_and_none_goto2() {
    let pc = 2;
    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Split);
    let hole = Hole::One(pc);
    let goto1 = None;
    let goto2 = None;

    let _result = compiler.fill_split(hole, goto1, goto2); // Should panic
}

#[test]
fn test_fill_split_with_many_holes() {
    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Split);
    let hole1 = Hole::One(0);
    let hole2 = Hole::One(1);
    let hole3 = Hole::Many(vec![hole1, hole2]);
    let goto1 = Some(3);
    let goto2 = Some(4);

    let result = compiler.fill_split(hole3, goto1, goto2);
    if let Hole::Many(holes) = result {
        assert_eq!(holes.len(), 2); // Expect 2 holes to return
    } else {
        panic!("Expected a Hole::Many variant");
    }
}

