// Answer 0

#[derive(Debug)]
struct MockError;

impl std::fmt::Display for MockError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mock serialization error")
    }
}

impl std::error::Error for MockError {}

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

trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<(), E>
    where
        S: Serializer<E>;
}

struct Serializer<E> {
    _marker: std::marker::PhantomData<E>,
}

impl<E> Serializer<E> {
    fn new() -> Self {
        Serializer {
            _marker: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_serialize_key_should_return_err_on_serialize_fail() {
    struct FailingSerializable;

    impl Serialize for FailingSerializable {
        fn serialize<S>(&self, _serializer: S) -> Result<(), MockError>
        where
            S: Serializer<MockError>,
        {
            Err(MockError)
        }
    }

    let mut serializer = Serializer::<MockError>::new();
    let key = FailingSerializable;

    let result = serializer.serialize_key(&key);
    assert!(result.is_err());
}

