// Answer 0

#[test]
fn test_new_creates_array_visitor() {
    use serde::de::ArrayVisitor;
    use std::marker::PhantomData;

    struct ArrayVisitor<T> {
        marker: PhantomData<T>,
    }

    impl<T> ArrayVisitor<T> {
        fn new() -> Self {
            ArrayVisitor {
                marker: PhantomData,
            }
        }
    }

    let visitor: ArrayVisitor<u8> = ArrayVisitor::new();
    assert!(visitor.marker == PhantomData::<u8>);
}

