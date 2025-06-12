// Answer 0

#[test]
fn test_compile_single_expression() {
    struct DummyCompiler {
        num_exprs: usize,
    }

    impl DummyCompiler {
        fn compile_one(&mut self, _expr: &Hir) -> Result<Program, Error> {
            Ok(Program { /* initialization */ })
        }

        fn compile_many(&mut self, _exprs: &[Hir]) -> Result<Program, Error> {
            Ok(Program { /* initialization */ })
        }
    }

    let exprs = vec![Hir::new(/* initialization */)];
    let mut compiler = DummyCompiler { num_exprs: 0 };

    let result = compile(&mut compiler, &exprs);
    assert!(result.is_ok());
}

#[test]
fn test_compile_multiple_expressions() {
    struct DummyCompiler {
        num_exprs: usize,
    }

    impl DummyCompiler {
        fn compile_one(&mut self, _expr: &Hir) -> Result<Program, Error> {
            Ok(Program { /* initialization */ })
        }

        fn compile_many(&mut self, _exprs: &[Hir]) -> Result<Program, Error> {
            Ok(Program { /* initialization */ })
        }
    }

    let exprs = vec![Hir::new(/* initialization */), Hir::new(/* initialization */)];
    let mut compiler = DummyCompiler { num_exprs: 0 };

    let result = compile(&mut compiler, &exprs);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_compile_empty_expression() {
    struct DummyCompiler {
        num_exprs: usize,
    }

    let exprs: Vec<Hir> = vec![];
    let mut compiler = DummyCompiler { num_exprs: 0 };

    let _result = compile(&mut compiler, &exprs);
}

