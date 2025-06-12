// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

#[derive(Debug)]
enum ClassSet {
    Item(Item),
    BinaryOp(BinaryOp),
}

#[derive(Debug)]
struct Item {
    span: Span,
}

impl Item {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
struct BinaryOp {
    span: Span,
}

#[test]
fn test_class_set_item_span() {
    let item = Item {
        span: Span::new(0, 10),
    };
    let class_set = ClassSet::Item(item);
    if let ClassSet::Item(ref x) = class_set {
        assert_eq!(x.span(), &Span::new(0, 10));
    } else {
        panic!("The class set should be of type Item");
    }
}

#[test]
fn test_class_set_binary_op_span() {
    let binary_op = BinaryOp {
        span: Span::new(5, 15),
    };
    let class_set = ClassSet::BinaryOp(binary_op);
    if let ClassSet::BinaryOp(ref x) = class_set {
        assert_eq!(x.span, Span::new(5, 15));
    } else {
        panic!("The class set should be of type BinaryOp");
    }
}

