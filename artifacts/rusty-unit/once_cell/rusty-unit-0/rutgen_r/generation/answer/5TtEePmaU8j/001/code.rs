// Answer 0

#[test]
fn test_fmt_with_empty_cell() {
    struct Lazy {
        cell: Option<i32>,
    }
    
    impl Lazy {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct("Lazy").field("cell", &self.cell).field("init", &"..").finish()
        }
    }

    let lazy = Lazy { cell: None };
    let result = format!("{:?}", lazy.fmt(&mut std::fmt::Formatter::new()));
    assert_eq!(result, "Lazy { cell: None, init: .. }");
}

#[test]
fn test_fmt_with_initialized_cell() {
    struct Lazy {
        cell: Option<i32>,
    }
    
    impl Lazy {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct("Lazy").field("cell", &self.cell).field("init", &"..").finish()
        }
    }

    let lazy = Lazy { cell: Some(42) };
    let result = format!("{:?}", lazy.fmt(&mut std::fmt::Formatter::new()));
    assert_eq!(result, "Lazy { cell: Some(42), init: .. }");
}

#[should_panic(expected = "failed formatting")]
#[test]
fn test_fmt_with_unexpected_input() {
    struct Lazy {
        cell: Option<i32>,
    }
    
    impl Lazy {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            // Deliberately causing a panic for testing
            panic!("failed formatting");
        }
    }

    let lazy = Lazy { cell: Some(42) };
    let _ = lazy.fmt(&mut std::fmt::Formatter::new());
}

