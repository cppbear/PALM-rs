// Answer 0

#[test]
fn test_pop_repetition() {
    struct DummyRepetition;
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let repetition_frame = Frame::Repetition(&DummyRepetition);
    let visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.pop(repetition_frame);
    assert_eq!(result, None);
}

#[test]
fn test_pop_group() {
    struct DummyGroup;
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let group_frame = Frame::Group(&DummyGroup);
    let visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.pop(group_frame);
    assert_eq!(result, None);
}

#[test]
fn test_pop_concat_with_empty_tail() {
    struct DummyAst;
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let concat_frame = Frame::Concat {
        head: &DummyAst,
        tail: &[],
    };
    let visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.pop(concat_frame);
    assert_eq!(result, None);
}

#[test]
fn test_pop_concat_with_non_empty_tail() {
    struct DummyAst;
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let ast_array: [&DummyAst; 2] = [&DummyAst, &DummyAst];
    let concat_frame = Frame::Concat {
        head: &ast_array[0],
        tail: &ast_array[1..],
    };
    let visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.pop(concat_frame);
    match result {
        Some(Frame::Concat { head, tail }) => {
            assert_eq!(head, &ast_array[1]);
            assert!(tail.is_empty());
        }
        _ => panic!("Expected Some(Frame::Concat)"),
    }
}

#[test]
fn test_pop_alternation_with_empty_tail() {
    struct DummyAst;
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let alternation_frame = Frame::Alternation {
        head: &DummyAst,
        tail: &[],
    };
    let visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.pop(alternation_frame);
    assert_eq!(result, None);
}

#[test]
fn test_pop_alternation_with_non_empty_tail() {
    struct DummyAst;
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let ast_array: [&DummyAst; 2] = [&DummyAst, &DummyAst];
    let alternation_frame = Frame::Alternation {
        head: &ast_array[0],
        tail: &ast_array[1..],
    };
    let visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.pop(alternation_frame);
    match result {
        Some(Frame::Alternation { head, tail }) => {
            assert_eq!(head, &ast_array[1]);
            assert!(tail.is_empty());
        }
        _ => panic!("Expected Some(Frame::Alternation)"),
    }
}

