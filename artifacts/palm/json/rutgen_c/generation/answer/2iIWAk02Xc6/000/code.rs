// Answer 0

#[test]
fn test_serialize_seq_empty() {
    struct TestFormatter;

    impl TestFormatter {
        fn begin_array(&mut self) -> Result<()> {
            Ok(())
        }

        fn end_array(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            Ok(())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let mut formatter = TestFormatter;
    
    let serializer: &mut Serializer<_, _> = &mut Serializer { writer, formatter };
    let result = serializer.serialize_seq(Some(0)).unwrap();

    if let Compound::Map { state, .. } = result {
        assert_eq!(state, State::Empty);
    } else {
        panic!("Expected Compound::Map for empty sequence.");
    }
}

#[test]
fn test_serialize_seq_non_empty() {
    struct TestFormatter;

    impl TestFormatter {
        fn begin_array(&mut self) -> Result<()> {
            Ok(())
        }

        fn end_array(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct TestWriter;

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            Ok(())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let mut formatter = TestFormatter;
    
    let serializer: &mut Serializer<_, _> = &mut Serializer { writer, formatter };
    let result = serializer.serialize_seq(Some(1)).unwrap();

    if let Compound::Map { state, .. } = result {
        assert_eq!(state, State::First);
    } else {
        panic!("Expected Compound::Map for non-empty sequence.");
    }
}

