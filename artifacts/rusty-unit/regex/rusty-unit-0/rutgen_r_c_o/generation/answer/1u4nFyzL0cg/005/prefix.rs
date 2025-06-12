// Answer 0

#[test]
fn test_parse_set_class_with_symmetric_difference() {
    let pattern = "[a-z~~]";
    let pos = Cell::new(0);
    let capture_index = Cell::new(0);
    let nest_limit = 10;
    let octal = false;
    let initial_ignore_whitespace = false;
    let ignore_whitespace = Cell::new(false);
    let comments = RefCell::new(vec![]);
    let stack_group = RefCell::new(vec![]);
    let stack_class = RefCell::new(vec![]);
    let capture_names = RefCell::new(vec![]);
    let scratch = RefCell::new(String::new());

    let parser = Parser {
        pos,
        capture_index,
        nest_limit,
        octal,
        initial_ignore_whitespace,
        ignore_whitespace,
        comments,
        stack_group,
        stack_class,
        capture_names,
        scratch,
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_i.parse_set_class();
}

#[test]
fn test_parse_set_class_with_nested_classes() {
    let pattern = "[[a-z][^A-Z]&&[0-9]]";
    let pos = Cell::new(0);
    let capture_index = Cell::new(0);
    let nest_limit = 10;
    let octal = false;
    let initial_ignore_whitespace = false;
    let ignore_whitespace = Cell::new(false);
    let comments = RefCell::new(vec![]);
    let stack_group = RefCell::new(vec![]);
    let stack_class = RefCell::new(vec![]);
    let capture_names = RefCell::new(vec![]);
    let scratch = RefCell::new(String::new());

    let parser = Parser {
        pos,
        capture_index,
        nest_limit,
        octal,
        initial_ignore_whitespace,
        ignore_whitespace,
        comments,
        stack_group,
        stack_class,
        capture_names,
        scratch,
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_i.parse_set_class();
}

#[test]
fn test_parse_set_class_with_empty_class() {
    let pattern = "[]";
    let pos = Cell::new(0);
    let capture_index = Cell::new(0);
    let nest_limit = 10;
    let octal = false;
    let initial_ignore_whitespace = false;
    let ignore_whitespace = Cell::new(false);
    let comments = RefCell::new(vec![]);
    let stack_group = RefCell::new(vec![]);
    let stack_class = RefCell::new(vec![]);
    let capture_names = RefCell::new(vec![]);
    let scratch = RefCell::new(String::new());

    let parser = Parser {
        pos,
        capture_index,
        nest_limit,
        octal,
        initial_ignore_whitespace,
        ignore_whitespace,
        comments,
        stack_group,
        stack_class,
        capture_names,
        scratch,
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_i.parse_set_class();
}

#[test]
#[should_panic]
fn test_parse_set_class_invalid_range() {
    let pattern = "[a-z-]";
    let pos = Cell::new(0);
    let capture_index = Cell::new(0);
    let nest_limit = 10;
    let octal = false;
    let initial_ignore_whitespace = false;
    let ignore_whitespace = Cell::new(false);
    let comments = RefCell::new(vec![]);
    let stack_group = RefCell::new(vec![]);
    let stack_class = RefCell::new(vec![]);
    let capture_names = RefCell::new(vec![]);
    let scratch = RefCell::new(String::new());

    let parser = Parser {
        pos,
        capture_index,
        nest_limit,
        octal,
        initial_ignore_whitespace,
        ignore_whitespace,
        comments,
        stack_group,
        stack_class,
        capture_names,
        scratch,
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let _ = parser_i.parse_set_class();
}

