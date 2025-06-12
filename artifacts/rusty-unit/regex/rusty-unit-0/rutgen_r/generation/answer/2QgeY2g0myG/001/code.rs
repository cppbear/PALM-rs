// Answer 0

#[test]
fn test_parser_borrow() {
    use std::cell::RefCell;
    
    struct TestStruct {
        parser: RefCell<Parser>,
    }

    struct Parser {
        value: usize,
    }

    impl TestStruct {
        fn parser(&self) -> &Parser {
            self.parser.borrow()
        }
    }

    // Test case: Normal usage
    let test_struct = TestStruct {
        parser: RefCell::new(Parser { value: 42 }),
    };

    let parser_ref = test_struct.parser();
    assert_eq!(parser_ref.value, 42);
}

#[test]
#[should_panic]
fn test_parser_borrow_panic() {
    use std::cell::RefCell;

    struct TestStruct {
        parser: RefCell<Parser>,
    }

    struct Parser {
        value: usize,
    }

    impl TestStruct {
        fn parser(&self) -> &Parser {
            self.parser.borrow()
        }
    }

    // Test case: Attempting to borrow while already borrowing
    let test_struct = TestStruct {
        parser: RefCell::new(Parser { value: 42 }),
    };

    let _first_borrow = test_struct.parser(); // First borrow
    let _second_borrow = test_struct.parser(); // This will cause panic
}

