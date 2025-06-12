// Answer 0

#[test]
fn test_pos() {
    struct DummyParser {
        pos: Position,
    }

    impl DummyParser {
        fn new(offset: usize, line: usize, column: usize) -> Self {
            Self {
                pos: Position { offset, line, column },
            }
        }
    }

    struct Dummy {
        parser: DummyParser,
    }

    impl Dummy {
        fn new(offset: usize, line: usize, column: usize) -> Self {
            Self {
                parser: DummyParser::new(offset, line, column),
            }
        }

        fn parser(&self) -> &DummyParser {
            &self.parser
        }
    }

    let dummy = Dummy::new(10, 1, 5);
    let position = dummy.pos();
    assert_eq!(position.offset, 10);
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 5);
}

