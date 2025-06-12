// Answer 0

#[test]
fn test_end_with_empty_state() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(0)
        }
        
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl ser::Formatter for MockFormatter {
        fn end_array(&mut self, _: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object_value(&mut self, _: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object(&mut self, _: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    
    let compound = Compound::Map { 
        ser: &mut Serializer { writer, formatter },
        state: State::Empty,
    };
    
    compound.end();
}

#[test]
fn test_end_with_valid_state() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> core::result::Result<usize, std::io::Error> {
            Ok(0)
        }
        
        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;
    impl ser::Formatter for MockFormatter {
        fn end_array(&mut self, _: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object_value(&mut self, _: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object(&mut self, _: &mut dyn io::Write) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    
    let compound = Compound::Map { 
        ser: &mut Serializer { writer, formatter },
        state: State::First,
    };
    
    compound.end();
}

