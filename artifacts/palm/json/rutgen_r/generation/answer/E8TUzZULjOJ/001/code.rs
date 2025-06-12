// Answer 0

#[test]
fn test_serialize_i16() {
    struct MockFormatter;

    impl MockFormatter {
        fn write_i16(&self, _: &mut Vec<u8>, value: i16) -> Result<(), std::io::Error> {
            // Mock success case
            Ok(())
        }
    }

    struct TestStruct {
        formatter: MockFormatter,
        writer: Vec<u8>,
    }

    impl TestStruct {
        fn serialize_i16(self, value: i16) -> Result<()> {
            self.formatter
                .write_i16(&mut self.writer, value)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }

    // Test case: Serialize a positive i16 value
    let test_struct = TestStruct {
        formatter: MockFormatter,
        writer: Vec::new(),
    };
    assert!(test_struct.serialize_i16(1234).is_ok());

    // Test case: Serialize a negative i16 value
    let test_struct_neg = TestStruct {
        formatter: MockFormatter,
        writer: Vec::new(),
    };
    assert!(test_struct_neg.serialize_i16(-1234).is_ok());

    // Test case: Serialize minimum i16 value
    let test_struct_min = TestStruct {
        formatter: MockFormatter,
        writer: Vec::new(),
    };
    assert!(test_struct_min.serialize_i16(i16::MIN).is_ok());

    // Test case: Serialize maximum i16 value
    let test_struct_max = TestStruct {
        formatter: MockFormatter,
        writer: Vec::new(),
    };
    assert!(test_struct_max.serialize_i16(i16::MAX).is_ok());
}

