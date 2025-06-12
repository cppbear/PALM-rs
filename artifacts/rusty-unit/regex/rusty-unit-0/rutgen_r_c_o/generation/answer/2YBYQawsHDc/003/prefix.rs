// Answer 0

#[test]
fn test_pop_with_empty_concat_tail() {
    struct MyVisitor;

    impl Visitor for MyVisitor {
        type Output = ();
        type Err = ();
    }

    let tail: Vec<Ast> = Vec::new();
    let frame = Frame::Concat {
        head: &Ast::Empty(Span::new(0, 0)),
        tail: &tail,
    };

    let visitor = MyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.pop(frame);
}

#[test]
fn test_pop_with_empty_tail_alternation() {
    struct MyVisitor;

    impl Visitor for MyVisitor {
        type Output = ();
        type Err = ();
    }

    let tail: Vec<Ast> = Vec::new();
    let frame = Frame::Alternation {
        head: &Ast::Empty(Span::new(0, 0)),
        tail: &tail,
    };

    let visitor = MyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.pop(frame);
}

