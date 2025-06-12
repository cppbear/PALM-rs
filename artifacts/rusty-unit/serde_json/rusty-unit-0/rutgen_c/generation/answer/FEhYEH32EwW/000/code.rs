// Answer 0

#[test]
fn test_serialize_element_map_first_element() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_array_value(&mut self, writer: &mut dyn io::Write, first: bool) -> Result<()> {
            if first {
                writer.write(&[b'['])?;
            }
            Ok(())
        }

        fn end_array_value(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(&[b']'])?;
            Ok(())
        }
    }

    let value = "test_value"; // Assuming we are serializing a string
    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;

    let mut compound = Compound::Map {
        ser: &mut Serializer {
            writer,
            formatter,
        },
        state: State::First,
    };

    let result = compound.serialize_element(&value);
    assert!(result.is_ok());
    assert_eq!(compound.ser.writer.output, b"[\"test_value\"]");
}

#[test]
fn test_serialize_element_map_subsequent_elements() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_array_value(&mut self, writer: &mut dyn io::Write, first: bool) -> Result<()> {
            if first {
                writer.write(&[b'['])?;
            }
            Ok(())
        }

        fn end_array_value(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(&[b']'])?;
            Ok(())
        }
    }

    let value1 = "first_value";
    let value2 = "second_value";
    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter;

    let mut compound = Compound::Map {
        ser: &mut Serializer {
            writer,
            formatter,
        },
        state: State::First,
    };

    // Serialize the first element
    compound.serialize_element(&value1).unwrap();

    // Change state to simulate subsequent element
    if let Compound::Map { ser, state } = &mut compound {
        *state = State::Rest;
    }

    // Serialize the second element
    let result = compound.serialize_element(&value2);
    assert!(result.is_ok());
    assert_eq!(compound.ser.writer.output, b"[\"first_value\",\"second_value\"]");
}

