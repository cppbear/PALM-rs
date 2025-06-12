// Answer 0

#[test]
fn test_pop_concat_non_empty_tail() {
    struct TestAst;
    let head = &TestAst;
    let tail = vec![&TestAst, &TestAst, &TestAst]; // tail has multiple elements

    let induct = Frame::Concat {
        head,
        tail: &tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(induct);

    match result {
        Some(Frame::Concat { head: result_head, tail: result_tail }) => {
            assert_eq!(result_head, &tail[0]);
            assert_eq!(result_tail, &tail[1..]);
        },
        _ => panic!("Expected Some(Frame::Concat) but got {:?}", result),
    }
}

#[test]
fn test_pop_concat_single_element_tail() {
    struct TestAst;
    let head = &TestAst;
    let tail = vec![&TestAst]; // tail has one element

    let induct = Frame::Concat {
        head,
        tail: &tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(induct);

    match result {
        Some(Frame::Concat { head: result_head, tail: result_tail }) => {
            assert_eq!(result_head, &tail[0]);
            assert_eq!(result_tail.len(), 0); // tail should be empty
        },
        _ => panic!("Expected Some(Frame::Concat) but got {:?}", result),
    }
}

#[test]
fn test_pop_alternation_non_empty_tail() {
    struct TestAst;
    let head = &TestAst;
    let tail = vec![&TestAst, &TestAst]; // tail has multiple elements

    let induct = Frame::Alternation {
        head,
        tail: &tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(induct);

    match result {
        Some(Frame::Alternation { head: result_head, tail: result_tail }) => {
            assert_eq!(result_head, &tail[0]);
            assert_eq!(result_tail, &tail[1..]);
        },
        _ => panic!("Expected Some(Frame::Alternation) but got {:?}", result),
    }
}

#[test]
fn test_pop_alternation_single_element_tail() {
    struct TestAst;
    let head = &TestAst;
    let tail = vec![&TestAst]; // tail has one element

    let induct = Frame::Alternation {
        head,
        tail: &tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(induct);

    match result {
        Some(Frame::Alternation { head: result_head, tail: result_tail }) => {
            assert_eq!(result_head, &tail[0]);
            assert_eq!(result_tail.len(), 0); // tail should be empty
        },
        _ => panic!("Expected Some(Frame::Alternation) but got {:?}", result),
    }
}

