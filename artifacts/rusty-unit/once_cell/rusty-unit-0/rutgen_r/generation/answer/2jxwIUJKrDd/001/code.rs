// Answer 0

#[test]
fn test_fmt_lazy_non_empty_cell() {
    struct Lazy {
        cell: String,
    }

    impl Lazy {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct("Lazy").field("cell", &self.cell).field("init", &"..").finish()
        }
    }

    let lazy_instance = Lazy {
        cell: String::from("Test Cell"),
    };
    
    let mut buffer = String::new();
    let result = lazy_instance.fmt(&mut std::fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(buffer, "Lazy { cell: \"Test Cell\", init: \"..\" }");
}

#[test]
fn test_fmt_lazy_empty_cell() {
    struct Lazy {
        cell: String,
    }

    impl Lazy {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct("Lazy").field("cell", &self.cell).field("init", &"..").finish()
        }
    }

    let lazy_instance = Lazy {
        cell: String::from(""),
    };
    
    let mut buffer = String::new();
    let result = lazy_instance.fmt(&mut std::fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(buffer, "Lazy { cell: \"\", init: \"..\" }");
}

#[should_panic]
#[test]
fn test_fmt_lazy_panic_condition() {
    struct Lazy {
        cell: Option<String>,
    }

    impl Lazy {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct("Lazy").field("cell", &self.cell).field("init", &"..").finish()
        }
    }

    let lazy_instance = Lazy {
        cell: None,
    };
    
    let mut buffer = String::new();
    lazy_instance.fmt(&mut std::fmt::Formatter::new(&mut buffer));
}

