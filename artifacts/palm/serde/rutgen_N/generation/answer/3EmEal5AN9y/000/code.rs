// Answer 0

#[test]
fn test_new() {
    use std::marker::PhantomData;

    struct ContentVisitor<T> {
        value: PhantomData<T>,
    }

    let visitor: ContentVisitor<()> = new();
    assert_eq!(std::mem::size_of_val(&visitor.value), std::mem::size_of::<PhantomData<()>>());
}

