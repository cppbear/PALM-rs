// Answer 0

#[test]
fn test_content_serializer_new() {
    use std::marker::PhantomData;

    struct MyError;

    struct MyContentSerializer {
        error: PhantomData<MyError>,
    }

    impl MyContentSerializer {
        pub fn new() -> Self {
            MyContentSerializer { error: PhantomData }
        }
    }

    let serializer: MyContentSerializer = MyContentSerializer::new();
    assert!(std::mem::size_of::<MyContentSerializer>() > 0); // ensuring something is created
}

