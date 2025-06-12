// Answer 0

#[derive(Default)]
struct Parser {
    ignore_whitespace: bool,
}

struct TestStruct {
    parser: Parser,
}

impl TestStruct {
    fn parser(&self) -> &Parser {
        &self.parser
    }
}

impl Parser {
    fn get(&self) -> bool {
        self.ignore_whitespace
    }
}

#[test]
fn test_ignore_whitespace_true() {
    let test_struct = TestStruct {
        parser: Parser {
            ignore_whitespace: true,
        },
    };
    assert!(test_struct.ignore_whitespace());
}

#[test]
fn test_ignore_whitespace_false() {
    let test_struct = TestStruct {
        parser: Parser {
            ignore_whitespace: false,
        },
    };
    assert!(!test_struct.ignore_whitespace());
}

