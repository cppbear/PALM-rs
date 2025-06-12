// Answer 0

#[derive(Debug)]
struct DummyParser {
    stack_group: std::cell::RefCell<Vec<GroupState>>,
}

impl DummyParser {
    fn new() -> Self {
        Self {
            stack_group: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl DummyParser {
    fn pos(&self) -> usize {
        0 // mock position for testing
    }
}

enum GroupState {
    Alternation(ast::Alternation),
}

mod ast {
    #[derive(Debug)]
    pub struct Concat {
        pub span: Span,
    }

    #[derive(Debug)]
    pub struct Alternation {
        pub span: Span,
        pub asts: Vec<Concat>,
    }

    #[derive(Debug)]
    pub struct Span {
        pub start: usize,
        pub end: usize,
    }

    impl Concat {
        pub fn into_ast(self) -> Self {
            self // in real implementation, this would transform into AST
        }
    }
}

struct TestStruct {
    parser: DummyParser,
}

impl TestStruct {
    fn new() -> Self {
        Self {
            parser: DummyParser::new(),
        }
    }

    fn push_or_add_alternation(&self, concat: ast::Concat) {
        use GroupState::*;

        let mut stack = self.parser.stack_group.borrow_mut();
        if let Some(&mut Alternation(ref mut alts)) = stack.last_mut() {
            alts.asts.push(concat.into_ast());
            return;
        }
        stack.push(Alternation(ast::Alternation {
            span: ast::Span {
                start: concat.span.start,
                end: self.parser.pos(),
            },
            asts: vec![concat.into_ast()],
        }));
    }
}

#[test]
fn test_push_or_add_to_existing_alternation() {
    let test_struct = TestStruct::new();

    // Setup initial state with an existing Alternation
    let initial_concat = ast::Concat {
        span: ast::Span { start: 0, end: 1 },
    };
    test_struct.push_or_add_alternation(initial_concat);

    let additional_concat = ast::Concat {
        span: ast::Span { start: 1, end: 2 },
    };
    test_struct.push_or_add_alternation(additional_concat);

    let stack = test_struct.parser.stack_group.borrow();
    assert_eq!(stack.len(), 1);
    if let Some(GroupState::Alternation(ref alts)) = stack.last() {
        assert_eq!(alts.asts.len(), 2);
    } else {
        panic!("Expected Alternation in stack");
    }
}

#[test]
fn test_push_first_alternation() {
    let test_struct = TestStruct::new();

    let first_concat = ast::Concat {
        span: ast::Span { start: 0, end: 1 },
    };
    test_struct.push_or_add_alternation(first_concat);

    let stack = test_struct.parser.stack_group.borrow();
    assert_eq!(stack.len(), 1);
    if let Some(GroupState::Alternation(ref alts)) = stack.last() {
        assert_eq!(alts.asts.len(), 1);
    } else {
        panic!("Expected Alternation in stack");
    }
}

