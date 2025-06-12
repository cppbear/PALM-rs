// Answer 0

#[test]
fn test_char_at_valid_position() {
    struct TestParser {
        pattern: String,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let parser_instance = TestParser {
        pattern: String::from("abc"),
    };
    let parser = ParserI::new(parser_instance, "abc");

    let result = parser.char_at(1);
    assert_eq!(result, 'b');
}

#[test]
#[should_panic(expected = "expected char at offset 5")]
fn test_char_at_invalid_position() {
    struct TestParser {
        pattern: String,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let parser_instance = TestParser {
        pattern: String::from("abc"),
    };
    let parser = ParserI::new(parser_instance, "abc");
    
    parser.char_at(5);
}

