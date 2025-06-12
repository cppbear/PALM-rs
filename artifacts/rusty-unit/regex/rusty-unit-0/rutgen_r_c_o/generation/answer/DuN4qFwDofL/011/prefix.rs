// Answer 0

#[test]
fn test_fill_split_with_hole_none() {
    let mut compiler = Compiler::new();
    let hole = Hole::None;
    let goto1 = None;
    let goto2 = None;
    compiler.fill_split(hole, goto1, goto2);
}

#[test]
fn test_fill_split_with_hole_none_and_some_goto() {
    let mut compiler = Compiler::new();
    let hole = Hole::None;
    let goto1 = Some(0);
    let goto2 = None;
    compiler.fill_split(hole, goto1, goto2);
}

#[test]
fn test_fill_split_with_hole_none_and_none_goto() {
    let mut compiler = Compiler::new();
    let hole = Hole::None;
    let goto1 = None;
    let goto2 = Some(0);
    compiler.fill_split(hole, goto1, goto2);
}

#[test]
fn test_fill_split_with_hole_none_and_both_goto() {
    let mut compiler = Compiler::new();
    let hole = Hole::None;
    let goto1 = Some(1);
    let goto2 = Some(2);
    compiler.fill_split(hole, goto1, goto2);
}

