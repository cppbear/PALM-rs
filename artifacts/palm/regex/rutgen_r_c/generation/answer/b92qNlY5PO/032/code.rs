// Answer 0

#[test]
fn test_cmd_literals_with_valid_args_and_prefixes() {
    use std::error::Error;

    #[derive(Debug)]
    struct MockHir;

    impl MockHir {
        fn new() -> Self {
            MockHir
        }
    }

    #[derive(Debug)]
    struct MockLiterals;

    impl MockLiterals {
        fn union_prefixes(&mut self, _e: &MockHir) -> Self {
            MockLiterals
        }

        fn unambiguous_prefixes(self) -> Self {
            self
        }

        fn literals(self) -> Vec<&'static str> {
            vec!["mock_literal1", "mock_literal2"]
        }
    }

    #[derive(Debug)]
    struct MockArgs {
        pub cmd_prefixes: bool,
        pub arg_patterns: Vec<String>,
        pub flag_all_literals: bool,
        pub flag_searcher: bool,
    }

    impl MockArgs {
        fn parse_many(&self) -> Result<Vec<MockHir>, Box<dyn Error + Send + Sync>> {
            Ok(self.arg_patterns.iter().map(|_| MockHir::new()).collect())
        }

        fn literals<F>(&self, exprs: &[MockHir], get_literals: F) -> MockLiterals
        where
            F: Fn(&mut MockLiterals, &MockHir) -> MockLiterals,
        {
            let mut lits = MockLiterals;
            for e in exprs {
                lits = get_literals(&mut lits, e);
            }
            lits
        }
    }

    let args = MockArgs {
        cmd_prefixes: true,
        arg_patterns: vec!["abc".to_string(), "def".to_string()],
        flag_all_literals: false,
        flag_searcher: true,
    };

    let result = cmd_literals(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_literals_with_valid_args_and_suffixes() {
    use std::error::Error;

    #[derive(Debug)]
    struct MockHir;

    impl MockHir {
        fn new() -> Self {
            MockHir
        }
    }

    #[derive(Debug)]
    struct MockLiterals;

    impl MockLiterals {
        fn union_suffixes(&mut self, _e: &MockHir) -> Self {
            MockLiterals
        }

        fn unambiguous_suffixes(self) -> Self {
            self
        }

        fn literals(self) -> Vec<&'static str> {
            vec!["mock_literal1", "mock_literal2"]
        }
    }

    #[derive(Debug)]
    struct MockArgs {
        pub cmd_prefixes: bool,
        pub arg_patterns: Vec<String>,
        pub flag_all_literals: bool,
        pub flag_searcher: bool,
    }

    impl MockArgs {
        fn parse_many(&self) -> Result<Vec<MockHir>, Box<dyn Error + Send + Sync>> {
            Ok(self.arg_patterns.iter().map(|_| MockHir::new()).collect())
        }

        fn literals<F>(&self, exprs: &[MockHir], get_literals: F) -> MockLiterals
        where
            F: Fn(&mut MockLiterals, &MockHir) -> MockLiterals,
        {
            let mut lits = MockLiterals;
            for e in exprs {
                lits = get_literals(&mut lits, e);
            }
            lits
        }
    }

    let args = MockArgs {
        cmd_prefixes: false,
        arg_patterns: vec!["abc".to_string(), "def".to_string()],
        flag_all_literals: false,
        flag_searcher: true,
    };

    let result = cmd_literals(&args);
    assert!(result.is_ok());
}

