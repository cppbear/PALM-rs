// Answer 0

#[test]
fn test_parse_set_class_open_with_negation_and_items() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            MockParser {
                input: input.chars().collect(),
                pos: 0,
            }
        }

        fn char(&self) -> char {
            *self.input.get(self.pos).unwrap_or(&'\u{0}')
        }

        fn pos(&self) -> usize {
            self.pos
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.pos < self.input.len() {
                self.pos += 1;
                true
            } else {
                false
            }
        }

        fn span_char(&self) -> usize {
            self.pos
        }

        fn span(&self) -> (usize, usize) {
            (self.pos, self.pos + 1)
        }

        fn error(&self, _span: (usize, usize), _kind: &str) -> String {
            "Error".to_string()
        }
    }

    let mut parser = MockParser::new("[^--");

    let result = parser.parse_set_class_open();

    assert!(result.is_ok(), "Expected Parse to succeed.");
    let (set, union) = result.unwrap();
    assert!(set.negated, "Expected set to be negated.");
    assert_eq!(union.items.len(), 2, "Expected union to have 2 items.");

    // Checking the first item is '-'
    if let ast::ClassSetItem::Literal(lit) = &union.items[0] {
        assert_eq!(lit.c, '-', "Expected first item to be '-'.");
    }

    // Checking the second item is another '-'
    if let ast::ClassSetItem::Literal(lit) = &union.items[1] {
        assert_eq!(lit.c, '-', "Expected second item to be '-'.");
    }
}

