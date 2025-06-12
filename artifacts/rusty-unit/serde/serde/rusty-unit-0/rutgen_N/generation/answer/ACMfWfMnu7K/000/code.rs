// Answer 0

#[derive(Debug)]
struct UnitDeserializer<T> {
    marker: std::marker::PhantomData<T>,
}

impl<T> UnitDeserializer<T> {
    pub fn new() -> Self {
        UnitDeserializer {
            marker: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_unit_deserializer_new() {
    let deserializer: UnitDeserializer<()> = UnitDeserializer::new();
    assert!(std::any::type_name::<UnitDeserializer<()>>() == "serde::UnitDeserializer");
}

