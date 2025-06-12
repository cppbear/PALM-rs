// Answer 0

#[test]
fn test_push_or_add_alternation_with_existing_alternation() {
    struct MockParser {
        stack_group: RefCell<Vec<GroupState>>,
        pos: Cell<Position>,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser {
                stack_group: RefCell::new(vec![]),
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            }
        }
        
        fn push_or_add_alternation(&self, concat: ast::Concat) {
            use self::GroupState::*;

            let mut stack = self.stack_group.borrow_mut();
            if let Some(&mut Alternation(ref mut alts)) = stack.last_mut() {
                alts.asts.push(concat.into_ast());
                return;
            }
            stack.push(Alternation(ast::Alternation {
                span: Span::new(concat.span.start, self.pos.get()),
                asts: vec![concat.into_ast()],
            }));
        }
    }

    let mut parser = MockParser::new();

    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 5, line: 1, column: 6 });
    let concat1 = Concat { span, asts: vec![] };
    
    parser.push_or_add_alternation(concat1.clone());

    let new_span = Span::new(Position { offset: 5, line: 1, column: 6 }, Position { offset: 10, line: 1, column: 11 });
    let concat2 = Concat { span: new_span, asts: vec![] };

    // Now the stack should have one alternation with one ast
    assert_eq!(parser.stack_group.borrow().len(), 1);
    if let GroupState::Alternation(ref alts) = parser.stack_group.borrow()[0] {
        assert_eq!(alts.asts.len(), 1);
    }

    // Now we add a second concat to the existing alternation
    parser.push_or_add_alternation(concat2.clone());

    // Now the alternation should have two asts
    if let GroupState::Alternation(ref alts) = parser.stack_group.borrow()[0] {
        assert_eq!(alts.asts.len(), 2);
    }
}

#[test]
fn test_push_or_add_alternation_with_empty_stack() {
    struct MockParser {
        stack_group: RefCell<Vec<GroupState>>,
        pos: Cell<Position>,
    }

    impl MockParser {
        fn new() -> Self {
            MockParser {
                stack_group: RefCell::new(vec![]),
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            }
        }

        fn push_or_add_alternation(&self, concat: ast::Concat) {
            use self::GroupState::*;

            let mut stack = self.stack_group.borrow_mut();
            if let Some(&mut Alternation(ref mut alts)) = stack.last_mut() {
                alts.asts.push(concat.into_ast());
                return;
            }
            stack.push(Alternation(ast::Alternation {
                span: Span::new(concat.span.start, self.pos.get()),
                asts: vec![concat.into_ast()],
            }));
        }
    }

    let mut parser = MockParser::new();

    let span = Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 5, line: 1, column: 6 });
    let concat = Concat { span, asts: vec![] };
    
    // When the stack is empty, we should create a new alternation
    parser.push_or_add_alternation(concat.clone());

    assert_eq!(parser.stack_group.borrow().len(), 1);
    if let GroupState::Alternation(ref alts) = parser.stack_group.borrow()[0] {
        assert_eq!(alts.asts.len(), 1);
    }
}

