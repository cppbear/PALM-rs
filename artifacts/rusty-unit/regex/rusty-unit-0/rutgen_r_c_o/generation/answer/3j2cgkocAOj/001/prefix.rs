// Answer 0

#[test]
fn test_visit_class_set_binary_op_pre_with_nest_limit_zero() {
    let span = Span { start: Position(0), end: Position(1) };
    let lhs = Box::new(ClassSetBinaryOp { span: span.clone(), kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::empty()), rhs: Box::new(ClassSet::empty()) });
    let rhs = Box::new(ClassSetBinaryOp { span: span.clone(), kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::empty()), rhs: Box::new(ClassSet::empty()) });
    let ast = ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Union, lhs, rhs };
    
    let parser = Parser { nest_limit: 0, ..Default::default() };
    let parser_i = ParserI { parser: &parser, pattern: "" };
    let mut nest_limiter = NestLimiter::new(&parser_i);
    
    nest_limiter.visit_class_set_binary_op_pre(&ast);
}

#[test]
fn test_visit_class_set_binary_op_pre_with_depth_at_limit() {
    let span = Span { start: Position(0), end: Position(1) };
    let lhs = Box::new(ClassSetBinaryOp { span: span.clone(), kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::empty()), rhs: Box::new(ClassSet::empty()) });
    let rhs = Box::new(ClassSetBinaryOp { span: span.clone(), kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::empty()), rhs: Box::new(ClassSet::empty()) });
    let ast = ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Union, lhs, rhs };
    
    let parser = Parser { nest_limit: 10, ..Default::default() };
    let parser_i = ParserI { parser: &parser, pattern: "" };
    let mut nest_limiter = NestLimiter { p: &parser_i, depth: 10 };
    
    nest_limiter.visit_class_set_binary_op_pre(&ast);
}

#[test]
#[should_panic]
fn test_visit_class_set_binary_op_pre_exceeds_nest_limit() {
    let span = Span { start: Position(0), end: Position(1) };
    let lhs = Box::new(ClassSetBinaryOp { span: span.clone(), kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::empty()), rhs: Box::new(ClassSet::empty()) });
    let rhs = Box::new(ClassSetBinaryOp { span: span.clone(), kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::empty()), rhs: Box::new(ClassSet::empty()) });
    let ast = ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Union, lhs, rhs };
    
    let parser = Parser { nest_limit: 5, ..Default::default() };
    let parser_i = ParserI { parser: &parser, pattern: "" };
    let mut nest_limiter = NestLimiter { p: &parser_i, depth: 6 };
    
    nest_limiter.visit_class_set_binary_op_pre(&ast);
}

#[test]
fn test_visit_class_set_binary_op_pre_with_various_depths() {
    let span = Span { start: Position(0), end: Position(2) };
    let lhs = Box::new(ClassSetBinaryOp { span: span.clone(), kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::empty()), rhs: Box::new(ClassSet::empty()) });
    let rhs = Box::new(ClassSetBinaryOp { span: span.clone(), kind: ClassSetBinaryOpKind::Union, lhs: Box::new(ClassSet::empty()), rhs: Box::new(ClassSet::empty()) });
    
    for depth in 0..11 {
        let ast = ClassSetBinaryOp { span, kind: ClassSetBinaryOpKind::Union, lhs, rhs };
        let parser = Parser { nest_limit: 10, ..Default::default() };
        let parser_i = ParserI { parser: &parser, pattern: "" };
        let mut nest_limiter = NestLimiter { p: &parser_i, depth };
        
        nest_limiter.visit_class_set_binary_op_pre(&ast);
    }
}

