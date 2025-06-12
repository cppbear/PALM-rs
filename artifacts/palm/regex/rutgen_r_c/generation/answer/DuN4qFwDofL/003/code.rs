// Answer 0

fn test_fill_split_hole_many_all_conditions() {
    struct InstStub;
    impl InstStub {
        fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {}
        fn half_fill_split_goto1(&mut self, _goto1: InstPtr) {}
        fn half_fill_split_goto2(&mut self, _goto2: InstPtr) {}
    }

    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Compiled(Inst::Split));

    let pc = 0;
    let hole_one = Hole::One(pc);
    let hole_many = Hole::Many(vec![hole_one.clone(), hole_one.clone()]);
    
    let goto1 = Some(InstPtr::new());
    let goto2 = Some(InstPtr::new());
    
    let result = compiler.fill_split(hole_many.clone(), goto1, goto2);
    
    assert!(matches!(result, Hole::Many(new_holes) if new_holes.len() == 2));
}

fn test_fill_split_hole_many_empty_condition() {
    struct InstStub;
    impl InstStub {
        fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {}
        fn half_fill_split_goto1(&mut self, _goto1: InstPtr) {}
        fn half_fill_split_goto2(&mut self, _goto2: InstPtr) {}
    }

    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Compiled(Inst::Split));

    let pc = 0;
    let hole_one = Hole::One(pc);
    let hole_many = Hole::Many(vec![hole_one.clone(), hole_one.clone()]);
    
    let result = compiler.fill_split(hole_many.clone(), None, None);
    
    assert!(matches!(result, Hole::Many(new_holes) if new_holes.len() == 2));
}

fn test_fill_split_hole_single_condition() {
    struct InstStub;
    impl InstStub {
        fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {}
        fn half_fill_split_goto1(&mut self, _goto1: InstPtr) {}
    }

    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Compiled(Inst::Split));

    let pc = 0;
    let hole_one = Hole::One(pc);
    let hole_many = Hole::Many(vec![hole_one.clone(), hole_one.clone()]);
    
    let goto1 = Some(InstPtr::new());
    let goto2 = None;
    
    let result = compiler.fill_split(hole_many.clone(), goto1, goto2);
    
    assert!(matches!(result, Hole::Many(new_holes) if new_holes.len() == 1));
    assert!(matches!(new_holes[0], Hole::One(_)));
}

fn test_fill_split_none_condition() {
    struct InstStub;
    impl InstStub {
        fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {}
    }

    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Compiled(Inst::Split));

    let hole_none = Hole::None;
    
    let result = compiler.fill_split(hole_none, Some(InstPtr::new()), Some(InstPtr::new()));
    
    assert!(matches!(result, Hole::None));
}

