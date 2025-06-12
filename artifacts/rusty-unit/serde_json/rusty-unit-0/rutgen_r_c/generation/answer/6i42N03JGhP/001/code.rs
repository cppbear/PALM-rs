// Answer 0

#[test]
fn test_serialize_u128_should_return_err_on_begin_string_failure() {
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W>(&self, _: &mut W) -> Result<()> {
            Err(Error) // Simulating failure
        }

        fn write_u128<W>(&self, _: &mut W, _: u128) -> Result<()> {
            Ok(()) // This won't be called
        }

        fn end_string<W>(&self, _: &mut W) -> Result<()> {
            Ok(()) // This won't be called
        }
    }

    struct MockWriter;

    struct MapKeySerializerTest<'a> {
        ser: &'a mut Serializer<MockWriter, MockFormatter>,
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };

    let map_key_serializer = MapKeySerializerTest { ser: &mut serializer };

    let result = map_key_serializer.ser.serialize_u128(12345678901234567890);
    assert!(result.is_err());
}

#[test]
fn test_serialize_u128_should_return_err_on_write_u128_failure() {
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W>(&self, _: &mut W) -> Result<()> {
            Ok(()) // Simulating success
        }

        fn write_u128<W>(&self, _: &mut W, _: u128) -> Result<()> {
            Err(Error) // Simulating failure
        }

        fn end_string<W>(&self, _: &mut W) -> Result<()> {
            Ok(()) // This won't be called
        }
    }

    struct MockWriter;

    struct MapKeySerializerTest<'a> {
        ser: &'a mut Serializer<MockWriter, MockFormatter>,
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };

    let map_key_serializer = MapKeySerializerTest { ser: &mut serializer };

    let result = map_key_serializer.ser.serialize_u128(12345678901234567890);
    assert!(result.is_err());
}

#[test]
fn test_serialize_u128_should_return_err_on_end_string_failure() {
    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W>(&self, _: &mut W) -> Result<()> {
            Ok(()) // Simulating success
        }

        fn write_u128<W>(&self, _: &mut W, _: u128) -> Result<()> {
            Ok(()) // Simulating success
        }

        fn end_string<W>(&self, _: &mut W) -> Result<()> {
            Err(Error) // Simulating failure
        }
    }

    struct MockWriter;

    struct MapKeySerializerTest<'a> {
        ser: &'a mut Serializer<MockWriter, MockFormatter>,
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };

    let map_key_serializer = MapKeySerializerTest { ser: &mut serializer };

    let result = map_key_serializer.ser.serialize_u128(12345678901234567890);
    assert!(result.is_err());
}

