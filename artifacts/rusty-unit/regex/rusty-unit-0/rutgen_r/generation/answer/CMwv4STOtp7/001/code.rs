// Answer 0

#[derive(Default)]
struct Parser {
    ignore_whitespace: std::cell::Cell<bool>,
}

struct TestStruct {
    parser: Parser,
}

impl TestStruct {
    fn parser(&self) -> &Parser {
        &self.parser
    }
}

#[test]
fn test_ignore_whitespace_true() {
    let test_struct = TestStruct {
        parser: Parser {
            ignore_whitespace: std::cell::Cell::new(true),
        },
    };
    
    assert!(test_struct.ignore_whitespace());
}

#[test]
fn test_ignore_whitespace_false() {
    let test_struct = TestStruct {
        parser: Parser {
            ignore_whitespace: std::cell::Cell::new(false),
        },
    };
    
    assert!(!test_struct.ignore_whitespace());
}

#[test]
fn test_ignore_whitespace_default() {
    let test_struct = TestStruct::default();
    
    assert!(!test_struct.ignore_whitespace());
}

