// Answer 0

#[test]
fn test_c_repeat_zero_or_one_greedy_success() {
    struct MockHir;
    struct MockInsts {
        insts: Vec<usize>,
    }

    impl MockInsts {
        fn new() -> Self {
            Self { insts: Vec::new() }
        }

        fn push_split_hole(&mut self) -> usize {
            self.insts.push(0);
            self.insts.len() - 1
        }

        fn fill_split(&mut self, _split: usize, _entry_rep: Option<usize>, _hole_rep: Option<usize>) -> usize {
            self.insts.push(1);
            self.insts.len() - 1
        }

        fn c(&self, _expr: &MockHir) -> Result<Patch, ()> {
            Ok(Patch { hole: Hole::Single(0), entry: 0 })
        }
    }

    struct Patch {
        hole: Hole,
        entry: usize,
    }

    enum Hole {
        Many(Vec<usize>),
        Single(usize),
    }

    let mut mock_insts = MockInsts::new();
    let expr = MockHir;
    let greedy = true;

    let result = mock_insts.c_repeat_zero_or_one(&expr, greedy);
    assert!(result.is_ok());

    if let Ok(patch) = result {
        assert!(matches!(patch.hole, Hole::Many(_)));
        assert_eq!(patch.entry, 0);
    }
}

