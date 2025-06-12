// Answer 0

fn test_fill_split_none() {
    let mut compiler = Compiler::new();
    let hole = Hole::None;
    let result = compiler.fill_split(hole, None, None);
    assert_eq!(result, Hole::None);
}

fn test_fill_split_one_full() {
    let mut compiler = Compiler::new();
    let pc = 0;
    compiler.insts.push(MaybeInst::Split);
    let hole = Hole::One(pc);
    let goto1 = Some(1);
    let goto2 = Some(2);
    let result = compiler.fill_split(hole, goto1, goto2);
    assert_eq!(result, Hole::None);
}

fn test_fill_split_many() {
    let mut compiler = Compiler::new();
    let pc = 0;
    compiler.insts.push(MaybeInst::Split);
    let hole1 = Hole::One(pc);
    let hole2 = Hole::One(pc);
    let holes = vec![hole1, hole2];
    let hole = Hole::Many(holes);
    let goto1 = Some(1);
    let goto2 = Some(2);
    let result = compiler.fill_split(hole, goto1, goto2);
    if let Hole::Many(h) = result {
        assert_eq!(h.len(), 0); // Expecting empty holes due to fills
    } else {
        panic!("Expected Hole::Many");
    }
}

fn test_fill_split_many_empty_holes() {
    let mut compiler = Compiler::new();
    let pc = 0;
    compiler.insts.push(MaybeInst::Split);
    let hole = Hole::Many(vec![Hole::None]);
    let goto1 = None;
    let goto2 = None;
    let result = compiler.fill_split(hole, goto1, goto2);
    assert_eq!(result, Hole::None);
}

fn test_fill_split_many_popping() {
    let mut compiler = Compiler::new();
    let pc = 0;
    compiler.insts.push(MaybeInst::Split);
    let hole1 = Hole::One(pc);
    let hole2 = Hole::One(pc);
    let hole3 = Hole::One(pc);
    let holes = vec![hole1, hole2, hole3];
    let hole = Hole::Many(holes);
    let goto1 = Some(1);
    let goto2 = Some(2);
    let _result = compiler.fill_split(hole, goto1, goto2);
    // Since new_holes.len() != 1, we can't assert like before. Need to check for panic.
}

