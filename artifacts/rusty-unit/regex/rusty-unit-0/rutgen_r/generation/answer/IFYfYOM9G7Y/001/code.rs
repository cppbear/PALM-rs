// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Item {
    span: Span,
}

impl Item {
    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
struct BinaryOp {
    span: Span,
}

#[derive(Debug)]
enum ClassSet {
    Item(Item),
    BinaryOp(BinaryOp),
}

impl ClassSet {
    pub fn span(&self) -> &Span {
        match *self {
            ClassSet::Item(ref x) => x.span(),
            ClassSet::BinaryOp(ref x) => &x.span,
        }
    }
}

#[test]
fn test_class_set_binary_op_span() {
    let binary_op = BinaryOp { span: Span { start: 5, end: 10 } };
    let class_set = ClassSet::BinaryOp(binary_op);
    let span = class_set.span();
    assert_eq!(span.start, 5);
    assert_eq!(span.end, 10);
}

#[test]
fn test_class_set_item_span_panics() {
    let item = Item { span: Span { start: 3, end: 7 } };
    let class_set = ClassSet::Item(item);
    let result = std::panic::catch_unwind(|| {
        let _ = class_set.span(); // This should not panic, but ensure it exists in context.
    });
    assert!(result.is_ok());
}

#[test]
fn test_class_set_binary_op_span_with_edge_values() {
    let binary_op = BinaryOp { span: Span { start: 0, end: 0 } };
    let class_set = ClassSet::BinaryOp(binary_op);
    let span = class_set.span();
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 0);
}

