// Answer 0

#[test]
fn test_end_with_valid_state() {
    struct MockWriter;
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct MockFormatter;
    
    impl ser::Formatter for MockFormatter {}
    
    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut compound = Compound::Map { ser: &mut Serializer { writer, formatter }, state: State::First };
    
    let result = compound.end();
    
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_end_with_empty_state() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    struct MockFormatter;
    
    impl ser::Formatter for MockFormatter {}
    
    let writer = MockWriter;
    let formatter = MockFormatter;
    let mut compound = Compound::Map { ser: &mut Serializer { writer, formatter }, state: State::Empty };
    
    let _result = compound.end(); // This should trigger a panic
}

