// Answer 0

#[test]
fn test_serialize_i64_success() {
    use serde_json::ser::{Serializer, Error};
    
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }

        fn get_output(self) -> Vec<u8> {
            self.output
        }
    }

    impl std::io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter::new();
    let serializer = Serializer::new(writer);
    
    let result = serializer.serialize_i64(42);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_i64_negative_value() {
    use serde_json::ser::{Serializer, Error};
    
    struct TestWriter {
        output: Vec<u8>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { output: Vec::new() }
        }

        fn get_output(self) -> Vec<u8> {
            self.output
        }
    }

    impl std::io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let writer = TestWriter::new();
    let serializer = Serializer::new(writer);
    
    let result = serializer.serialize_i64(-42);
    assert!(result.is_ok());
}

#[should_panic]
#[test]
fn test_serialize_i64_panic_on_write_error() {
    use serde_json::ser::{Serializer, Error};
    
    struct FailingWriter;

    impl std::io::Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }
        
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let writer = FailingWriter;
    let serializer = Serializer::new(writer);
    
    // This should panic due to the write error
    let _ = serializer.serialize_i64(42);
}

