// Answer 0

#[test]
fn test_c_repeat_zero_or_one_greedy() {
    struct MockHir;
    
    struct MockInsts {
        insts: Vec<String>,
    }

    impl MockInsts {
        fn new() -> Self {
            MockInsts { insts: Vec::new() }
        }
        
        fn push_split_hole(&mut self) -> usize {
            self.insts.push("split_hole".to_string());
            self.insts.len() - 1
        }
        
        fn fill_split(&mut self, split: usize, entry_rep: Option<usize>, _hole_rep: Option<usize>) -> usize {
            self.insts.push(format!("fill_split, entry_rep: {:?}, hole_rep: {:?}", entry_rep, None));
            self.insts.len() - 1
        }
        
        fn c(&self, _expr: &MockHir) -> Result<Patch, &'static str> {
            Ok(Patch { hole: Hole::Single(1), entry: 2 })
        }
    }

    struct Patch {
        hole: Hole,
        entry: usize,
    }
    
    enum Hole {
        Single(usize),
        Many(Vec<usize>),
    }

    let mut insts = MockInsts::new();
    let expr = MockHir;

    let result = insts.c_repeat_zero_or_one(&expr, true).unwrap();

    assert_eq!(result.entry, 0);
    if let Hole::Many(ref holes) = result.hole {
        assert_eq!(holes.len(), 2);
    } else {
        panic!("Expected a Many variant for hole.");
    }
}

#[test]
fn test_c_repeat_zero_or_one_non_greedy() {
    struct MockHir;
    
    struct MockInsts {
        insts: Vec<String>,
    }

    impl MockInsts {
        fn new() -> Self {
            MockInsts { insts: Vec::new() }
        }
        
        fn push_split_hole(&mut self) -> usize {
            self.insts.push("split_hole".to_string());
            self.insts.len() - 1
        }
        
        fn fill_split(&mut self, split: usize, _entry_rep: Option<usize>, hole_rep: Option<usize>) -> usize {
            self.insts.push(format!("fill_split, hole_rep: {:?}", hole_rep));
            self.insts.len() - 1
        }
        
        fn c(&self, _expr: &MockHir) -> Result<Patch, &'static str> {
            Ok(Patch { hole: Hole::Single(1), entry: 2 })
        }
    }

    struct Patch {
        hole: Hole,
        entry: usize,
    }
    
    enum Hole {
        Single(usize),
        Many(Vec<usize>),
    }

    let mut insts = MockInsts::new();
    let expr = MockHir;

    let result = insts.c_repeat_zero_or_one(&expr, false).unwrap();

    assert_eq!(result.entry, 0);
    if let Hole::Many(ref holes) = result.hole {
        assert_eq!(holes.len(), 2);
    } else {
        panic!("Expected a Many variant for hole.");
    }
}

