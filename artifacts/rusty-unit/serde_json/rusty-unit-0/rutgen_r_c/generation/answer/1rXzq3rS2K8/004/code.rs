// Answer 0

#[test]
fn test_serialize_f32_infinite_value() {
    struct TestFormatter;
    
    impl TestFormatter {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut Vec<u8>, _value: f32) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = Vec::new();
    let formatter = TestFormatter;

    let mut serializer = MapKeySerializer {
        ser: &mut Serializer { writer, formatter },
    };

    let result = serializer.serialize_f32(f32::INFINITY);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), float_key_must_be_finite());

    let result_nan = serializer.serialize_f32(f32::NAN);
    assert!(result_nan.is_err());
    assert_eq!(result_nan.err().unwrap(), float_key_must_be_finite());
}

