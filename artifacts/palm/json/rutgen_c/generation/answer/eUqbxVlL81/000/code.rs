// Answer 0

#[test]
fn test_serialize_unit_variant() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> { Ok(buf.len()) }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct MockFormatter;

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    {
        let map_key_serializer = MapKeySerializer { ser: &mut serializer };
        let result = map_key_serializer.serialize_unit_variant("Test", 0, "VariantA");
        assert!(result.is_ok());
    }

    {
        let map_key_serializer = MapKeySerializer { ser: &mut serializer };
        let result = map_key_serializer.serialize_unit_variant("Test", 1, "VariantB");
        assert!(result.is_ok());
    }
    
    {
        let map_key_serializer = MapKeySerializer { ser: &mut serializer };
        let result = map_key_serializer.serialize_unit_variant("Test", 0, "");
        assert!(result.is_ok()); // Edge case with empty variant name
    }
}

