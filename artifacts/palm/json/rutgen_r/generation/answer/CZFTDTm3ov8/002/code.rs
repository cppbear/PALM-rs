// Answer 0

fn test_serialize_u8_success() -> Result<()> {
    struct MockFormatter;
    struct MockWriter;
    struct Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    }
    
    impl MockFormatter {
        fn begin_string(&self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }
        
        fn write_u8(&self, _: &mut MockWriter, _: u8) -> Result<()> {
            Ok(())
        }

        fn end_string(&self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    let serializer = Serializer {
        formatter: MockFormatter,
        writer: MockWriter,
    };
    
    let result = serializer.serialize_u8(42);
    assert!(result.is_ok());
    
    Ok(())
}

fn test_serialize_u8_write_u8_error() -> Result<()> {
    struct MockErrorWriter;
    struct ErrorSerializer {
        formatter: MockFormatter,
        writer: MockErrorWriter,
    }
    
    impl MockErrorWriter {
        fn write_u8(&self, _: &mut MockWriter, _: u8) -> Result<()> {
            Err(Error::io)
        }
    }
    
    impl MockFormatter {
        fn begin_string(&self, _: &mut MockErrorWriter) -> Result<()> {
            Ok(())
        }

        fn end_string(&self, _: &mut MockErrorWriter) -> Result<()> {
            Ok(())
        }
    }

    let serializer = ErrorSerializer {
        formatter: MockFormatter,
        writer: MockErrorWriter,
    };
    
    let result = serializer.serialize_u8(42);
    assert!(result.is_err());
    
    Ok(())
}

fn test_serialize_u8_begin_string_error() -> Result<()> {
    struct ErrorBeginWriter;
    struct ErrorSerializer {
        formatter: ErrorFormatter,
        writer: ErrorBeginWriter,
    }
    
    struct ErrorFormatter;

    impl ErrorFormatter {
        fn begin_string(&self, _: &mut ErrorBeginWriter) -> Result<()> {
            Err(Error::io)
        }

        fn write_u8(&self, _: &mut ErrorBeginWriter, _: u8) -> Result<()> {
            Ok(())
        }

        fn end_string(&self, _: &mut ErrorBeginWriter) -> Result<()> {
            Ok(())
        }
    }

    let serializer = ErrorSerializer {
        formatter: ErrorFormatter,
        writer: ErrorBeginWriter,
    };
    
    let result = serializer.serialize_u8(42);
    assert!(result.is_err());
    
    Ok(())
}

