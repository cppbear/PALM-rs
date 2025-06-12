// Answer 0

#[test]
fn test_unit_deserializer_new_std() {
    let deserializer: UnitDeserializer<ErrorImpl> = UnitDeserializer::new();
}

#[test]
fn test_unit_deserializer_new_no_alloc() {
    let deserializer: UnitDeserializer<()> = UnitDeserializer::new();
}

#[test]
fn test_unit_deserializer_new_edge_case() {
    let deserializer: UnitDeserializer<ErrorImpl> = UnitDeserializer::new();
}

