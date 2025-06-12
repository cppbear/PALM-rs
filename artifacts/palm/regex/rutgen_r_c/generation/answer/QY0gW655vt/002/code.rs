// Answer 0

fn test_c_alternate_empty_sub_expression() {
    use syntax::hir::{Hir, HirKind};

    struct TestCompiler {
        compiler: Compiler,
    }

    impl TestCompiler {
        fn new() -> Self {
            TestCompiler {
                compiler: Compiler::new(),
            }
        }

        fn compile_alternation(&mut self, exprs: Vec<Hir>) -> Result {
            self.compiler.c_alternate(&exprs)
        }

        fn create_empty_hir(&self) -> Hir {
            Hir::new(HirKind::Empty) // assuming there's a method to create an empty Hir
        }

        fn create_non_empty_hir(&self) -> Hir {
            Hir::new(HirKind::Literal(hir::Literal::Unicode('a'))) // example for non-empty expression
        }
    }

    let mut test_compiler = TestCompiler::new();

    let exprs = vec![
        test_compiler.create_empty_hir(), // This is the empty sub-expression
        test_compiler.create_non_empty_hir(), // A valid sub-expression
    ];

    let result = test_compiler.compile_alternation(exprs);
    assert_eq!(result, Err(Error::Syntax(
        "alternations cannot currently contain \
         empty sub-expressions".to_string()
    )));
}

