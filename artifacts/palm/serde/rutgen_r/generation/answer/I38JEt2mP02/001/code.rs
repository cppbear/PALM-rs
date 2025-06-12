// Answer 0

#[test]
fn test_serialize_element_returns_err_on_serialize_failure() {
    use serde::ser::{Serializer, Serialize};
    use std::marker::PhantomData;

    struct MockSerializer<E> {
        elements: Vec<Result<(), E>>,
        _marker: PhantomData<E>,
    }

    impl<E> MockSerializer<E> {
        fn new() -> Self {
            Self {
                elements: Vec::new(),
                _marker: PhantomData,
            }
        }

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), E>
        where
            T: ?Sized + Serialize,
        {
            let value = value.serialize(ContentSerializer::<E>::new()).map_err(|err| {
                self.elements.push(Err(err));
                err
            })?;
            self.elements.push(Ok(()));
            Ok(())
        }
    }

    struct ContentSerializer<E> {
        _marker: PhantomData<E>,
    }

    impl<E> ContentSerializer<E> {
        fn new() -> Self {
            Self {
                _marker: PhantomData,
            }
        }
    }

    struct FailingSerializer;

    impl Serialize for FailingSerializer {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(S::Error::custom("serialization error"))
        }
    }

    let mut serializer = MockSerializer::new();
    let failing_value = FailingSerializer;

    let result = serializer.serialize_element(&failing_value);
    
    assert!(result.is_err());
}

