// Answer 0

#[test]
fn test_compiler_with_default_size_limit() {
    struct TestArgs {
        flag_size_limit: usize,
    }
    
    impl TestArgs {
        fn new(size_limit: usize) -> Self {
            Self { flag_size_limit: size_limit }
        }
        
        fn compiler(&self) -> Compiler {
            Compiler::new().size_limit(self.flag_size_limit)
        }
    }

    let args = TestArgs::new(10485760); // Default size limit
    let compiler = args.compiler();
    assert_eq!(compiler.size_limit(), 10485760);
}

#[test]
fn test_compiler_with_custom_size_limit() {
    struct TestArgs {
        flag_size_limit: usize,
    }
    
    impl TestArgs {
        fn new(size_limit: usize) -> Self {
            Self { flag_size_limit: size_limit }
        }
        
        fn compiler(&self) -> Compiler {
            Compiler::new().size_limit(self.flag_size_limit)
        }
    }

    let args = TestArgs::new(20480000); // Custom size limit
    let compiler = args.compiler();
    assert_eq!(compiler.size_limit(), 20480000);
}

#[test]
fn test_compiler_with_zero_size_limit() {
    struct TestArgs {
        flag_size_limit: usize,
    }
    
    impl TestArgs {
        fn new(size_limit: usize) -> Self {
            Self { flag_size_limit: size_limit }
        }
        
        fn compiler(&self) -> Compiler {
            Compiler::new().size_limit(self.flag_size_limit)
        }
    }

    let args = TestArgs::new(0); // Edge case - zero size limit
    let compiler = args.compiler();
    assert_eq!(compiler.size_limit(), 0);
}

