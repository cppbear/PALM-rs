// Answer 0

#[derive(Default)]
struct SerializeTuple<E> {
    elements: Vec<E>,
    error: std::marker::PhantomData<E>,
}

struct Serializer;

impl Serializer {
    fn serialize_tuple(self, len: usize) -> Result<SerializeTuple<()>, ()> {
        Ok(SerializeTuple {
            elements: Vec::with_capacity(len),
            error: std::marker::PhantomData,
        })
    }
}

#[test]
fn test_serialize_tuple_zero_length() {
    let serializer = Serializer;
    let result = serializer.serialize_tuple(0).unwrap();
    assert_eq!(result.elements.capacity(), 0);
}

#[test]
fn test_serialize_tuple_non_zero_length() {
    let serializer = Serializer;
    let result = serializer.serialize_tuple(10).unwrap();
    assert_eq!(result.elements.capacity(), 10);
}

