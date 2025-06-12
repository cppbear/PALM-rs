// Answer 0

#[test]
fn test_pattern_valid() {
    struct TestParser;

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            static PATTERN: &str = "abc";
            static POS: Cell<Position> = Cell::new(Position::default());
            static CAPTURE_INDEX: Cell<u32> = Cell::new(0);
            static IGNORE_WHITESPACE: Cell<bool> = Cell::new(false);
            static COMMENTS: RefCell<Vec<ast::Comment>> = RefCell::new(Vec::new());
            static STACK_GROUP: RefCell<Vec<GroupState>> = RefCell::new(Vec::new());
            static STACK_CLASS: RefCell<Vec<ClassState>> = RefCell::new(Vec::new());
            static CAPTURE_NAMES: RefCell<Vec<ast::CaptureName>> = RefCell::new(Vec::new());
            static SCRATCH: RefCell<String> = RefCell::new(String::new());
            static NEST_LIMIT: u32 = 10;
            static OCTAL: bool = false;
            static INITIAL_IGNORE_WHITESPACE: bool = false;

            static PARSER: Parser = Parser {
                pos: POS,
                capture_index: CAPTURE_INDEX,
                nest_limit: NEST_LIMIT,
                octal: OCTAL,
                initial_ignore_whitespace: INITIAL_IGNORE_WHITESPACE,
                ignore_whitespace: IGNORE_WHITESPACE,
                comments: COMMENTS,
                stack_group: STACK_GROUP,
                stack_class: STACK_CLASS,
                capture_names: CAPTURE_NAMES,
                scratch: SCRATCH,
            };

            &PARSER
        }
    }

    let parser_instance = ParserI::new(TestParser, "abc");
    assert_eq!(parser_instance.pattern(), "abc");
}

#[test]
fn test_pattern_empty() {
    struct TestParser;

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            static PATTERN: &str = "";
            static POS: Cell<Position> = Cell::new(Position::default());
            static CAPTURE_INDEX: Cell<u32> = Cell::new(0);
            static IGNORE_WHITESPACE: Cell<bool> = Cell::new(false);
            static COMMENTS: RefCell<Vec<ast::Comment>> = RefCell::new(Vec::new());
            static STACK_GROUP: RefCell<Vec<GroupState>> = RefCell::new(Vec::new());
            static STACK_CLASS: RefCell<Vec<ClassState>> = RefCell::new(Vec::new());
            static CAPTURE_NAMES: RefCell<Vec<ast::CaptureName>> = RefCell::new(Vec::new());
            static SCRATCH: RefCell<String> = RefCell::new(String::new());
            static NEST_LIMIT: u32 = 10;
            static OCTAL: bool = false;
            static INITIAL_IGNORE_WHITESPACE: bool = false;

            static PARSER: Parser = Parser {
                pos: POS,
                capture_index: CAPTURE_INDEX,
                nest_limit: NEST_LIMIT,
                octal: OCTAL,
                initial_ignore_whitespace: INITIAL_IGNORE_WHITESPACE,
                ignore_whitespace: IGNORE_WHITESPACE,
                comments: COMMENTS,
                stack_group: STACK_GROUP,
                stack_class: STACK_CLASS,
                capture_names: CAPTURE_NAMES,
                scratch: SCRATCH,
            };

            &PARSER
        }
    }

    let parser_instance = ParserI::new(TestParser, "");
    assert_eq!(parser_instance.pattern(), "");
}

