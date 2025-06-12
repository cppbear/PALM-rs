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
fn test_class_set_item_span() {
    let item = Item {
        span: Span { start: 0, end: 5 },
    };
    let class_set = ClassSet::Item(item);
    
    let expected_span = Span { start: 0, end: 5 };
    assert_eq!(class_set.span(), &expected_span);
}

#[test]
fn test_class_set_binary_op_span() {
    let binary_op = BinaryOp {
        span: Span { start: 1, end: 3 },
    };
    let class_set = ClassSet::BinaryOp(binary_op);
    
    let expected_span = Span { start: 1, end: 3 };
    assert_eq!(class_set.span(), &expected_span);
}

#[test]
fn test_class_set_empty_span() {
    let binary_op = BinaryOp {
        span: Span { start: 0, end: 0 },
    };
    let class_set = ClassSet::BinaryOp(binary_op);
    
    let expected_span = Span { start: 0, end: 0 };
    assert_eq!(class_set.span(), &expected_span);
}

