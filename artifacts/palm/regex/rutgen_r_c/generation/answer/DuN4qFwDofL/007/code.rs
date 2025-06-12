// Answer 0

#[test]
fn test_fill_split_with_both_goto_filled() {
    struct TestInst {
        filled: bool,
    }
    
    impl TestInst {
        fn new() -> Self {
            TestInst { filled: false }
        }
    
        fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {
            self.filled = true;
        }
    
        fn half_fill_split_goto1(&mut self, _goto1: InstPtr) {
            self.filled = true;
        }
    
        fn half_fill_split_goto2(&mut self, _goto2: InstPtr) {
            self.filled = true;
        }
    }
    
    struct TestCompiler {
        insts: Vec<TestInst>,
    }
    
    impl TestCompiler {
        fn new() -> Self {
            TestCompiler {
                insts: vec![TestInst::new()],
            }
        }
    
        fn fill_split(&mut self, hole: Hole, goto1: Option<InstPtr>, goto2: Option<InstPtr>) -> Hole {
            match hole {
                Hole::One(pc) => {
                    match (goto1, goto2) {
                        (Some(goto1), Some(goto2)) => {
                            self.insts[pc].fill_split(goto1, goto2);
                            Hole::None
                        },
                        (Some(goto1), None) => {
                            self.insts[pc].half_fill_split_goto1(goto1);
                            Hole::One(pc)
                        },
                        (None, Some(goto2)) => {
                            self.insts[pc].half_fill_split_goto2(goto2);
                            Hole::One(pc)
                        },
                        (None, None) => unreachable!(),
                    }
                },
                Hole::None => Hole::None,
                Hole::Many(_) => unreachable!(),
            }
        }
    }

    let mut compiler = TestCompiler::new();
    let hole = Hole::One(0);
    let goto1 = Some(1);
    let goto2 = Some(2);
    
    let result = compiler.fill_split(hole, goto1, goto2);
    
    assert_eq!(result, Hole::None);
    assert!(compiler.insts[0].filled);
}

#[test]
#[should_panic]
fn test_fill_split_panic_on_none() {
    struct TestInst {
        filled: bool,
    }
    
    impl TestInst {
        fn new() -> Self {
            TestInst { filled: false }
        }
    
        fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {
            self.filled = true;
        }
    
        fn half_fill_split_goto1(&mut self, _goto1: InstPtr) {
            self.filled = true;
        }
    
        fn half_fill_split_goto2(&mut self, _goto2: InstPtr) {
            self.filled = true;
        }
    }
    
    struct TestCompiler {
        insts: Vec<TestInst>,
    }
    
    impl TestCompiler {
        fn new() -> Self {
            TestCompiler {
                insts: vec![TestInst::new()],
            }
        }
    
        fn fill_split(&mut self, hole: Hole, goto1: Option<InstPtr>, goto2: Option<InstPtr>) -> Hole {
            match hole {
                Hole::One(pc) => {
                    match (goto1, goto2) {
                        (Some(goto1), Some(goto2)) => {
                            self.insts[pc].fill_split(goto1, goto2);
                            Hole::None
                        },
                        (Some(goto1), None) => {
                            self.insts[pc].half_fill_split_goto1(goto1);
                            Hole::One(pc)
                        },
                        (None, Some(goto2)) => {
                            self.insts[pc].half_fill_split_goto2(goto2);
                            Hole::One(pc)
                        },
                        (None, None) => unreachable!(),
                    }
                },
                Hole::None => Hole::None,
                Hole::Many(_) => unreachable!(),
            }
        }
    }

    let mut compiler = TestCompiler::new();
    let hole = Hole::One(0);
    let goto1 = None;
    let goto2 = None;
    
    compiler.fill_split(hole, goto1, goto2); // This should panic.
}

