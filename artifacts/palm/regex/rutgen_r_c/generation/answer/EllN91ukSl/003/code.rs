// Answer 0

#[test]
fn test_pop_concat_empty_tail() {
    struct MockHir;
    let head = MockHir;
    let tail: Vec<MockHir> = Vec::new(); // Empty tail

    let induct = Frame::Concat {
        head: &head,
        tail: &tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(induct);

    assert_eq!(result, None);
}

#[test]
fn test_pop_concat_non_empty_tail() {
    struct MockHir;
    let head = MockHir;
    let tail: Vec<MockHir> = vec![MockHir, MockHir]; // Non-empty tail

    let induct = Frame::Concat {
        head: &head,
        tail: &tail,
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop(induct);

    assert_eq!(result, Some(Frame::Concat {
        head: &tail[0],
        tail: &tail[1..],
    }));
}

