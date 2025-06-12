// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    struct TestParser {
        ignore_whitespace: Cell<bool>,
    }
    
    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            unsafe { std::mem::transmute(self) }
        }
    }
    
    let test_parser = TestParser {
        ignore_whitespace: Cell::new(true),
    };
    
    let parser_i = ParserI::new(test_parser, ".*");
    assert!(parser_i.ignore_whitespace());
}

#[test]
fn test_ignore_whitespace_false() {
    struct TestParser {
        ignore_whitespace: Cell<bool>,
    }
    
    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            unsafe { std::mem::transmute(self) }
        }
    }
    
    let test_parser = TestParser {
        ignore_whitespace: Cell::new(false),
    };
    
    let parser_i = ParserI::new(test_parser, ".*");
    assert!(!parser_i.ignore_whitespace());
}

