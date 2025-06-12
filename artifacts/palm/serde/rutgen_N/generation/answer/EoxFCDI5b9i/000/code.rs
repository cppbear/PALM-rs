// Answer 0

#[test]
fn test_content_serializer_new() {
    struct ContentSerializer {
        error: std::marker::PhantomData<()>,
    }

    impl ContentSerializer {
        pub fn new() -> Self {
            ContentSerializer { error: std::marker::PhantomData }
        }
    }

    let serializer = ContentSerializer::new();
    assert!(std::mem::discriminant(&serializer.error) == std::mem::discriminant(&std::marker::PhantomData::<()>));
}

