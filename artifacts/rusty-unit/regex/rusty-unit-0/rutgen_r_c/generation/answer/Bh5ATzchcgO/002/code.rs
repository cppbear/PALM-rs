// Answer 0

#[test]
fn test_push_or_add_alternation_with_existing_alternation() {
    struct MockParser {
        stack_group: RefCell<Vec<GroupState>>,
        pos: Cell<Position>,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let existing_alternation_span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 5, line: 1, column: 6 });
    let existing_alternation = ast::Alternation {
        span: existing_alternation_span,
        asts: vec![ast::Ast::Empty(existing_alternation_span)],
    };

    let concat_span = Span::new(Position { offset: 6, line: 1, column: 7 }, Position { offset: 10, line: 1, column: 11 });
    let concat = ast::Concat {
        span: concat_span,
        asts: vec![ast::Ast::Empty(concat_span)],
    };

    let mut parser = MockParser {
        stack_group: RefCell::new(vec![GroupState::Alternation(existing_alternation)]),
        pos: Cell::new(Position { offset: 11, line: 1, column: 12 }),
    };

    let parser_instance = ParserI::new(&parser, "some_pattern");

    parser_instance.push_or_add_alternation(concat);

    let stack = parser.stack_group.borrow();
    if let Some(GroupState::Alternation(ref alts)) = stack.last() {
        assert_eq!(alts.asts.len(), 2);
    } else {
        panic!("Expected an Alternation to be present in the stack.");
    }
}

#[test]
fn test_push_or_add_alternation_without_existing_alternation() {
    struct MockParser {
        stack_group: RefCell<Vec<GroupState>>,
        pos: Cell<Position>,
    }

    impl Borrow<Parser> for MockParser {
        fn borrow(&self) -> &Parser {
            unimplemented!()
        }
    }

    let concat_span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 4, line: 1, column: 5 });
    let concat = ast::Concat {
        span: concat_span,
        asts: vec![ast::Ast::Empty(concat_span)],
    };

    let mut parser = MockParser {
        stack_group: RefCell::new(vec![]), // no existing alternation
        pos: Cell::new(Position { offset: 5, line: 1, column: 6 }),
    };

    let parser_instance = ParserI::new(&parser, "some_pattern");

    parser_instance.push_or_add_alternation(concat);

    let stack = parser.stack_group.borrow();
    assert_eq!(stack.len(), 1);
    if let Some(GroupState::Alternation(ref alts)) = stack.last() {
        assert_eq!(alts.asts.len(), 1);
    } else {
        panic!("Expected an Alternation to be present in the stack.");
    }
}

