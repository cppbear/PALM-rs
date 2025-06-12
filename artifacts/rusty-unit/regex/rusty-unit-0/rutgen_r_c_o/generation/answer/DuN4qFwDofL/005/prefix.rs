// Answer 0

#[test]
fn test_fill_split_hole_many_with_one() {
    let mut compiler = Compiler::new();
    let hole = Hole::Many(vec![Hole::None, Hole::One(0)]);
    let goto1 = Some(0);
    let goto2 = Some(1);
    compiler.fill_split(hole, goto1, goto2);
}

#[test]
fn test_fill_split_hole_many_with_multiple() {
    let mut compiler = Compiler::new();
    let hole = Hole::Many(vec![Hole::One(0), Hole::One(1), Hole::One(2)]);
    let goto1 = Some(0);
    let goto2 = Some(1);
    compiler.fill_split(hole, goto1, goto2);
}

#[test]
#[should_panic]
fn test_fill_split_hole_many_with_none() {
    let mut compiler = Compiler::new();
    let hole = Hole::Many(vec![Hole::None, Hole::None]);
    let goto1 = None;
    let goto2 = None;
    compiler.fill_split(hole, goto1, goto2);
}

#[test]
fn test_fill_split_hole_many_empty() {
    let mut compiler = Compiler::new();
    let hole = Hole::Many(vec![]);
    let goto1 = Some(0);
    let goto2 = Some(1);
    let result = compiler.fill_split(hole, goto1, goto2);
    assert!(matches!(result, Hole::None));
}

#[test]
fn test_fill_split_hole_many_single() {
    let mut compiler = Compiler::new();
    let hole = Hole::Many(vec![Hole::One(0)]);
    let goto1 = Some(0);
    let goto2 = Some(1);
    let result = compiler.fill_split(hole, goto1, goto2);
    assert!(matches!(result, Hole::One(0)));
}

