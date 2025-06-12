// Answer 0

#[test]
fn test_unit_deserializer_into_deserializer() {
    let deserializer: UnitDeserializer<Error> = UnitDeserializer { marker: PhantomData };
    let _ = deserializer.into_deserializer();
}

#[test]
fn test_unit_deserializer_into_deserializer_multiple_calls() {
    let deserializer: UnitDeserializer<Error> = UnitDeserializer { marker: PhantomData };
    let _ = deserializer.into_deserializer();
    let _ = deserializer.into_deserializer();
}

#[test]
fn test_unit_deserializer_into_deserializer_on_empty_instance() {
    let deserializer: UnitDeserializer<Error> = UnitDeserializer { marker: PhantomData };
    let _ = deserializer.into_deserializer();
}

