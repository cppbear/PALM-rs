// Answer 0

#[test]
fn test_c_class_bytes_single_range() {
    use hir::{ClassBytesRange};

    struct TestCompiler {
        compiler: Compiler,
    }

    impl TestCompiler {
        fn new() -> Self {
            Self {
                compiler: Compiler::new(),
            }
        }

        fn compile_class_bytes(&mut self, ranges: Vec<ClassBytesRange>) -> Result {
            self.compiler.c_class_bytes(&ranges)
        }
    }

    let mut test_compiler = TestCompiler::new();
    let range = ClassBytesRange::new(1, 5);
    let result = test_compiler.compile_class_bytes(vec![range]);

    assert!(result.is_ok());
}

#[test]
fn test_c_class_bytes_multiple_ranges() {
    use hir::{ClassBytesRange};

    struct TestCompiler {
        compiler: Compiler,
    }

    impl TestCompiler {
        fn new() -> Self {
            Self {
                compiler: Compiler::new(),
            }
        }

        fn compile_class_bytes(&mut self, ranges: Vec<ClassBytesRange>) -> Result {
            self.compiler.c_class_bytes(&ranges)
        }
    }

    let mut test_compiler = TestCompiler::new();
    let range1 = ClassBytesRange::new(1, 3);
    let range2 = ClassBytesRange::new(5, 7);
    let result = test_compiler.compile_class_bytes(vec![range1, range2]);

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_c_class_bytes_empty_range() {
    use hir::{ClassBytesRange};

    struct TestCompiler {
        compiler: Compiler,
    }

    impl TestCompiler {
        fn new() -> Self {
            Self {
                compiler: Compiler::new(),
            }
        }

        fn compile_class_bytes(&mut self, ranges: Vec<ClassBytesRange>) -> Result {
            self.compiler.c_class_bytes(&ranges)
        }
    }

    let mut test_compiler = TestCompiler::new();
    test_compiler.compile_class_bytes(vec![]); // This should panic due to empty ranges
}

