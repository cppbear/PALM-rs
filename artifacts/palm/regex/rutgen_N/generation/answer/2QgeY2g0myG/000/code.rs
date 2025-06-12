// Answer 0

#[test]
fn test_parser() {
    struct MockParser {
        state: String,
    }

    struct MockStruct {
        parser: std::cell::RefCell<MockParser>,
    }

    impl MockStruct {
        fn new(state: String) -> Self {
            MockStruct {
                parser: std::cell::RefCell::new(MockParser { state }),
            }
        }

        fn parser(&self) -> &MockParser {
            self.parser.borrow()
        }
    }

    let mock = MockStruct::new("initial state".to_string());
    let parser_ref = mock.parser();
    assert_eq!(parser_ref.state, "initial state");
}

