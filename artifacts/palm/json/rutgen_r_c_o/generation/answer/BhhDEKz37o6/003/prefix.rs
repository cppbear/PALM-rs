// Answer 0

#[test]
fn test_end_with_state_first() {
    struct TestWriter {
        buffer: Vec<u8>,
    }
    
    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let formatter = CompactFormatter::new(); // Assuming CompactFormatter is available
    let ser = Serializer { writer: &mut writer, formatter };
    
    let compound = Compound::Map {
        ser: &mut ser,
        state: State::First,
    };
    
    let result = compound.end();
    // No assertions as per instructions
}

#[test]
fn test_end_with_state_rest() {
    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let formatter = CompactFormatter::new(); // Assuming CompactFormatter is available
    let ser = Serializer { writer: &mut writer, formatter };
    
    let compound = Compound::Map {
        ser: &mut ser,
        state: State::Rest,
    };
    
    let result = compound.end();
    // No assertions as per instructions
}

