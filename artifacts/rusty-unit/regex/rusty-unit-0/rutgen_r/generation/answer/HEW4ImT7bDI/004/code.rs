// Answer 0

#[test]
fn test_c_concat_with_single_expression() {
    struct TestHir;
    
    struct TestCompile {
        insts: Vec<u8>,
    }

    impl TestCompile {
        fn c(&mut self, _: &TestHir) -> Result<Patch, ()> {
            Ok(Patch { hole: Hole::None, entry: self.insts.len() })
        }

        fn fill(&mut self, _: Hole, _: usize) {}
    }

    let mut compile = TestCompile { insts: vec![1, 2, 3] };
    let exprs = vec![&TestHir];

    let result = compile.c_concat(exprs.iter());

    assert!(result.is_ok());
}

#[test]
fn test_c_concat_with_multiple_expressions() {
    struct TestHir;

    struct TestCompile {
        insts: Vec<u8>,
    }

    impl TestCompile {
        fn c(&mut self, _: &TestHir) -> Result<Patch, ()> {
            Ok(Patch { hole: Hole::None, entry: self.insts.len() })
        }

        fn fill(&mut self, _: Hole, _: usize) {}
    }

    let mut compile = TestCompile { insts: vec![1, 2, 3] };
    let exprs = vec![&TestHir, &TestHir];

    let result = compile.c_concat(exprs.iter());

    assert!(result.is_ok());
}

#[test]
fn test_c_concat_with_empty_expressions() {
    struct TestCompile {
        insts: Vec<u8>,
    }

    impl TestCompile {
        fn c(&mut self, _: &TestHir) -> Result<Patch, ()> {
            Ok(Patch { hole: Hole::None, entry: self.insts.len() })
        }

        fn fill(&mut self, _: Hole, _: usize) {}
    }

    let mut compile = TestCompile { insts: vec![1, 2, 3] };
    let exprs: Vec<&TestHir> = Vec::new();

    let result = compile.c_concat(exprs.iter());

    assert!(result.is_ok());
}

