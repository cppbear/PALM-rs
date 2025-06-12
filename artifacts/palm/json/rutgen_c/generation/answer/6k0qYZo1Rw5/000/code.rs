// Answer 0

#[test]
fn test_end() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    let writer = MockWriter;
    let formatter = MockFormatter;
    
    let mut compound = Compound::Map { 
        ser: &mut Serializer { writer, formatter }, 
        state: State::Empty 
    };

    // Call the method we are testing
    let result = compound.end();
    
    // Check that the result is Ok
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_with_invalid_state() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    let writer = MockWriter;
    let formatter = MockFormatter;

    let mut compound = Compound::Map { 
        ser: &mut Serializer { writer, formatter }, 
        state: State::Rest // Invalid state for this test
    };

    // Calling end should panic due to invalid state
    let _ = compound.end();
}

