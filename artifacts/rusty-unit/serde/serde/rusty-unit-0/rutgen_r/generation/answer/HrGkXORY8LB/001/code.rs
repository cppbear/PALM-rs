// Answer 0

#[test]
fn test_serialize_element_err() {
    use serde::ser::{Serializer, Serialize};

    struct ContentSerializer<E> {
        _marker: std::marker::PhantomData<E>,
    }

    impl<E> ContentSerializer<E> {
        fn new() -> Self {
            ContentSerializer {
                _marker: std::marker::PhantomData,
            }
        }
    }

    struct MockSerializer;

    impl Serialize for MockSerializer {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(ErrorMock)
        }
    }

    struct ErrorMock;

    impl std::fmt::Display for ErrorMock {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }

    impl std::error::Error for ErrorMock {}

    struct TestStruct<E> {
        elements: Vec<Result<(), E>>,
    }

    impl<E> TestStruct<E> {
        fn new() -> Self {
            TestStruct {
                elements: Vec::new(),
            }
        }

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let value = value.serialize(ContentSerializer::<E>::new()).map_err(|_| todo!())?;
            self.elements.push(Ok(value));
            Ok(())
        }
    }

    let mut test_struct = TestStruct::<ErrorMock>::new();
    let result = test_struct.serialize_element(&MockSerializer);

    assert!(result.is_err());
}

