// Answer 0

#[test]
fn test_begin_array_value_with_false_first() {
    use std::io;
    use std::io::Cursor;

    struct TestWriter {
        cursor: Cursor<Vec<u8>>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                cursor: Cursor::new(Vec::new()),
            }
        }

        fn get_written_data(self) -> Vec<u8> {
            self.cursor.into_inner()
        }
    }

    let mut writer = TestWriter::new();
    let first = false;
    let mut ser = serde_json::ser::Serializer::new(writer.cursor.clone());
    ser.current_indent = 0; // Set initial indent
    ser.indent = 2; // Set desired indent

    let result = ser.begin_array_value(&mut writer.cursor, first);
    
    assert!(result.is_ok());
    let written_data = writer.get_written_data();
    assert_eq!(written_data, b",\n"); // Check the output for non-first array value
}

