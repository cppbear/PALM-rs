// Answer 0

#[test]
fn test_fill_split_hole_many_empty() {
    struct MockInst {
        filled: bool,
    }

    impl MockInst {
        fn new() -> Self {
            MockInst { filled: false }
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

    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Compiled(MockInst::new()));

    let hole = Hole::Many(vec![]); // Constraint: holes is empty
    let goto1 = Some(InstPtr::default());
    let goto2 = Some(InstPtr::default());

    let result = compiler.fill_split(hole, goto1, goto2);
    assert_eq!(result, Hole::None); // Expectation: should return Hole::None
}

#[test]
fn test_fill_split_hole_many_non_empty() {
    struct MockInst {
        filled: bool,
    }

    impl MockInst {
        fn new() -> Self {
            MockInst { filled: false }
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

    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Compiled(MockInst::new()));

    let hole = Hole::Many(vec![Hole::None, Hole::None]); // Use two empty holes
    let goto1 = Some(InstPtr::default());
    let goto2 = Some(InstPtr::default());

    let result = compiler.fill_split(hole, goto1, goto2);
    assert_eq!(result, Hole::None); // Expectation: should return Hole::None
}

