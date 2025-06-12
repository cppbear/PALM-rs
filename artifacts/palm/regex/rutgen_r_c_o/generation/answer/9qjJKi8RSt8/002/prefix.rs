// Answer 0

#[test]
fn test_fill_with_many_holes() {
    let mut compiler = Compiler::new();
    let mut insts = vec![MaybeInst::Uncompiled(InstHole::new()); 1000]; // Initial 1000 holes
    compiler.insts = insts;

    let holes = vec![Hole::One(0), Hole::One(1), Hole::One(2)];
    let hole = Hole::Many(holes);

    let goto = 10;
    compiler.fill(hole, goto);
}

#[test]
fn test_fill_with_many_holes_large() {
    let mut compiler = Compiler::new();
    let mut insts = vec![MaybeInst::Uncompiled(InstHole::new()); 1000]; // Initial 1000 holes
    compiler.insts = insts;

    let holes: Vec<Hole> = (0..1000).map(|i| Hole::One(i)).collect();
    let hole = Hole::Many(holes);

    let goto = 20;
    compiler.fill(hole, goto);
}

#[test]
fn test_fill_with_many_empty_holes() {
    let mut compiler = Compiler::new();
    let mut insts = vec![MaybeInst::Uncompiled(InstHole::new()); 1000]; // Initial 1000 holes
    compiler.insts = insts;

    let holes = vec![];
    let hole = Hole::Many(holes);

    let goto = 30;
    compiler.fill(hole, goto);
}

#[test]
fn test_fill_with_single_hole_in_many() {
    let mut compiler = Compiler::new();
    let mut insts = vec![MaybeInst::Uncompiled(InstHole::new()); 1000]; // Initial 1000 holes
    compiler.insts = insts;

    let holes = vec![Hole::One(0), Hole::One(1)];
    let hole = Hole::Many(holes);

    let goto = 40;
    compiler.fill(hole, goto);
}

#[test]
fn test_fill_with_non_existent_one_hole() {
    let mut compiler = Compiler::new();
    let mut insts = vec![MaybeInst::Uncompiled(InstHole::new()); 1000]; // Initial 1000 holes
    compiler.insts = insts;

    let holes = vec![Hole::One(999)]; // Valid index
    let hole = Hole::Many(holes);

    let goto = 50;
    compiler.fill(hole, goto);
}

