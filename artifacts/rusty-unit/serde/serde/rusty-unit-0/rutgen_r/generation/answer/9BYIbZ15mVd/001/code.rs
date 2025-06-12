// Answer 0

#[derive(Serialize)]
struct FaultyData;

impl Serialize for FaultyData {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(S::Error::custom("serialization error"))
    }
}

struct TestSerializer<E> {
    fields: Vec<Result<E, String>>,
}

impl<E> TestSerializer<E> {
    fn new() -> Self {
        TestSerializer { fields: Vec::new() }
    }

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), E>
    where
        T: ?Sized + Serialize,
    {
        let value = value.serialize(ContentSerializer::<E>::new())?;
        self.fields.push(value);
        Ok(())
    }
}

#[test]
#[should_panic(expected = "serialization error")]
fn test_serialize_field_err() {
    let mut serializer: TestSerializer<String> = TestSerializer::new();
    let faulty = FaultyData;
    
    let result = serializer.serialize_field(&faulty);
    assert!(result.is_err());
}

