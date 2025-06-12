// Answer 0

#[test]
fn test_visit_class_set_item_pre_union() {
    let mut parser = Parser {
        pos: Cell::new(Position(0)),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let span = Span { start: Position(1), end: Position(5) };
    let class_set_item = ast::ClassSetItem::Union(ClassSetUnion {
        span,
        items: vec![ClassSetItem::Literal(Literal('a')), ClassSetItem::Literal(Literal('b'))],
    });
    
    let mut nest_limiter = NestLimiter::new(&ParserI {
        parser: &parser,
        pattern: "^[ab]+$",
    });
    
    let _ = nest_limiter.visit_class_set_item_pre(&class_set_item);
}

#[test]
fn test_visit_class_set_item_pre_union_with_large_item_list() {
    let mut parser = Parser {
        pos: Cell::new(Position(0)),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let span = Span { start: Position(1), end: Position(1000) };
    let items = (1..=1000).map(|i| ClassSetItem::Literal(Literal(char::from(i + b'a')))).collect();
    
    let class_set_item = ast::ClassSetItem::Union(ClassSetUnion {
        span,
        items,
    });
    
    let mut nest_limiter = NestLimiter::new(&ParserI {
        parser: &parser,
        pattern: "^[a-z]{1,1000}$",
    });
    
    let _ = nest_limiter.visit_class_set_item_pre(&class_set_item);
}

#[test]
#[should_panic]
fn test_visit_class_set_item_pre_union_exceed_depth_limit() {
    let mut parser = Parser {
        pos: Cell::new(Position(0)),
        capture_index: Cell::new(0),
        nest_limit: 1,
        octal: false,
        initial_ignore_whitespace: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };
    
    let span = Span { start: Position(1), end: Position(10) };
    let class_set_item = ast::ClassSetItem::Union(ClassSetUnion {
        span,
        items: vec![
            ClassSetItem::Bracketed(Box::new(ClassBracketed {
                span,
                negated: false,
                kind: ClassSet::Normal,
            })),
            ClassSetItem::Bracketed(Box::new(ClassBracketed {
                span,
                negated: false,
                kind: ClassSet::Normal,
            }))
        ],
    });
    
    let mut nest_limiter = NestLimiter::new(&ParserI {
        parser: &parser,
        pattern: "^(?:[a-z]|[A-Z])+",
    });
    
    let _ = nest_limiter.visit_class_set_item_pre(&class_set_item);
}

