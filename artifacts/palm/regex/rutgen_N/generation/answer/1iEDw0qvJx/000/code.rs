// Answer 0

#[test]
fn test_new_parser() {
    struct Parser;

    struct ParserBuilder;

    impl ParserBuilder {
        pub fn new() -> Self {
            ParserBuilder
        }

        pub fn build(self) -> Parser {
            Parser
        }
    }

    let parser = ParserBuilder::new().build();
    assert!(std::mem::size_of_val(&parser) > 0); // Check that parser is not zero-sized
}

