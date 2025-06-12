// Answer 0

#[derive(Debug)]
struct ArrayVisitor {
    marker: std::marker::PhantomData<()>,
}

impl ArrayVisitor {
    fn new() -> Self {
        ArrayVisitor {
            marker: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_array_visitor_new() {
    let visitor = ArrayVisitor::new();
    assert_eq!(format!("{:?}", visitor.marker), "PhantomData");
}

