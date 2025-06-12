// Answer 0

#[test]
fn test_c_repeat_range_min_not_equal_max() {
    struct DummyHir;
    struct DummyCompiler {
        hole: usize,
        entry: usize,
    }

    impl DummyCompiler {
        fn c_concat<I>(&mut self, _: I) -> Result<Patch, ()>
        where
            I: Iterator<Item = &'_ Hir>,
        {
            Ok(Patch { hole: self.hole, entry: self.entry })
        }
        fn fill_to_next(&mut self, _: usize) {}
        fn push_split_hole(&mut self) -> usize { 0 }
        fn fill_split(&mut self, _: usize, _: Option<usize>, _: Option<usize>) -> usize { 0 }
        fn c(&mut self, _: &Hir) -> Result<Patch, ()> { 
            Ok(Patch { hole: 0, entry: 1 }) 
        }
    }

    struct Patch {
        hole: usize,
        entry: usize,
    }

    struct Hole;

    impl Hole {
        fn Many(h: Vec<usize>) -> Hole { Hole }
    }

    let mut compiler = DummyCompiler { hole: 0, entry: 1 };
    let expr = DummyHir;
    let min = 2;
    let max = 5;
    let greedy = true;

    let result = compiler.c_repeat_range(&expr, greedy, min, max);
    
    assert!(result.is_ok());
    if let Ok(patch) = result {
        assert_eq!(patch.entry, compiler.entry);
    }
}

#[test]
fn test_c_repeat_range_empty_range() {
    struct DummyHir;
    struct DummyCompiler {
        hole: usize,
        entry: usize,
    }

    impl DummyCompiler {
        fn c_concat<I>(&mut self, _: I) -> Result<Patch, ()>
        where
            I: Iterator<Item = &'_ Hir>,
        {
            Ok(Patch { hole: self.hole, entry: self.entry })
        }
        fn fill_to_next(&mut self, _: usize) {}
        fn push_split_hole(&mut self) -> usize { 0 }
        fn fill_split(&mut self, _: usize, _: Option<usize>, _: Option<usize>) -> usize { 0 }
        fn c(&mut self, _: &Hir) -> Result<Patch, ()> { 
            Ok(Patch { hole: 0, entry: 1 }) 
        }
    }

    struct Patch {
        hole: usize,
        entry: usize,
    }

    struct Hole;

    impl Hole {
        fn Many(h: Vec<usize>) -> Hole { Hole }
    }

    let mut compiler = DummyCompiler { hole: 0, entry: 1 };
    let expr = DummyHir;
    let min = 0;
    let max = 0; // min == max, should not reach this.
    let greedy = true;

    let result = compiler.c_repeat_range(&expr, greedy, min, max);
    
    assert!(result.is_ok());
    if let Ok(patch) = result {
        assert_eq!(patch.entry, compiler.entry);
    }
}

