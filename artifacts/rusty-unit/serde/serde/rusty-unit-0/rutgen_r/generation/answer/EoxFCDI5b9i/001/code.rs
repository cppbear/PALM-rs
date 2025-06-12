// Answer 0

#[test]
fn test_content_serializer_new() {
    struct ContentSerializer<T> {
        error: std::marker::PhantomData<T>,
    }

    impl<T> ContentSerializer<T> {
        pub fn new() -> Self {
            ContentSerializer { error: std::marker::PhantomData }
        }
    }

    let serializer: ContentSerializer<()> = ContentSerializer::new();
    assert_eq!(std::mem::size_of::<ContentSerializer<()>>(), std::mem::size_of::<std::marker::PhantomData<()>>());
}

#[test]
fn test_content_serializer_new_with_different_type() {
    struct ContentSerializer<T> {
        error: std::marker::PhantomData<T>,
    }

    impl<T> ContentSerializer<T> {
        pub fn new() -> Self {
            ContentSerializer { error: std::marker::PhantomData }
        }
    }

    let serializer: ContentSerializer<i32> = ContentSerializer::new();
    assert_eq!(std::mem::size_of::<ContentSerializer<i32>>(), std::mem::size_of::<std::marker::PhantomData<i32>>());
}

