// Answer 0

#[test]
fn test_fill_with_hole_none() {
    let mut compiler = Compiler::new();
    let hole = Hole::None;
    let goto = 0;
    compiler.fill(hole, goto);
}

#[test]
fn test_fill_with_hole_one() {
    let mut compiler = Compiler::new();
    let pc = 5; // Valid pc within bounds
    compiler.insts.resize(6, MaybeInst::Uncompiled(InstHole));
    let hole = Hole::One(pc);
    let goto = 1;
    compiler.fill(hole, goto);
}

#[test]
fn test_fill_with_hole_many() {
    let mut compiler = Compiler::new();
    let pcs = vec![3, 4]; // Valid pcs within bounds
    compiler.insts.resize(5, MaybeInst::Uncompiled(InstHole));
    let holes = pcs.into_iter().map(|pc| Hole::One(pc)).collect::<Vec<_>>();
    let hole = Hole::Many(holes);
    let goto = 2;
    compiler.fill(hole, goto);
}

