// Answer 0

#[test]
fn test_end_array_without_value() -> Result<()> {
    use alloc::vec::Vec;
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

        fn get_result(self) -> String {
            String::from_utf8(self.cursor.into_inner()).unwrap()
        }
    }

    let indent: &[u8] = b"    "; // Example indentation
    let mut formatter = PrettyFormatter {
        current_indent: 1, // Start with 1 indentation
        has_value: false, // Constraint: has_value is false
        indent,
    };

    let mut writer = TestWriter::new();

    // Calling end_array with the provided writer
    formatter.end_array(&mut writer)?;

    // Check the output, expecting only "]" with no indentation or newline
    assert_eq!(writer.get_result(), b"]".to_vec());

    Ok(())
}

