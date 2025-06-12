// Answer 0

#[test]
fn test_fill_split_none_hole() {
    let mut compiler = Compiler::new();
    let result = compiler.fill_split(Hole::None, None, None);
    assert_eq!(result, Hole::None);
}

#[test]
fn test_fill_split_one_hole_some_goto1() {
    let mut compiler = Compiler::new();
    let hole = Hole::One(0);
    compiler.insts.push(MaybeInst::Split);
    let result = compiler.fill_split(hole, Some(1), None);
    assert_eq!(result, Hole::One(0));
}

#[test]
fn test_fill_split_one_hole_some_goto2() {
    let mut compiler = Compiler::new();
    let hole = Hole::One(0);
    compiler.insts.push(MaybeInst::Split);
    let result = compiler.fill_split(hole, None, Some(2));
    assert_eq!(result, Hole::One(0));
}

#[test]
#[should_panic]
fn test_fill_split_one_hole_none() {
    let mut compiler = Compiler::new();
    let hole = Hole::One(0);
    compiler.insts.push(MaybeInst::Split);
    let _result = compiler.fill_split(hole, None, None);
}

#[test]
fn test_fill_split_many_holes() {
    let mut compiler = Compiler::new();
    let hole = Hole::Many(vec![Hole::One(0), Hole::One(1)]);
    compiler.insts.push(MaybeInst::Split);
    compiler.insts.push(MaybeInst::Split);
    let result = compiler.fill_split(hole, Some(1), Some(2));
    match result {
        Hole::Many(ref holes) => {
            assert_eq!(holes.len(), 2);
        }
        _ => panic!("Expected Hole::Many"),
    }
}

