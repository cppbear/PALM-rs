// Answer 0

#[test]
fn test_serialize_seq_zero_length() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_array(&self, _writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_array(&self, _writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "end_array error"))
        }
    }

    struct Serializer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl Serializer {
        fn new() -> Self {
            Self {
                writer: Vec::new(),
                formatter: MockFormatter,
            }
        }

        fn serialize_seq(self, len: Option<usize>) -> Result<(), std::io::Error> {
            self.formatter.begin_array(&mut self.writer)?;
            if len == Some(0) {
                self.formatter.end_array(&mut self.writer)?;
            }
            Ok(())
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_seq(Some(0));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::Other);
}

#[test]
fn test_serialize_seq_non_zero_length() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_array(&self, _writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_array(&self, _writer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "end_array error"))
        }
    }

    struct Serializer {
        writer: Vec<u8>,
        formatter: MockFormatter,
    }

    impl Serializer {
        fn new() -> Self {
            Self {
                writer: Vec::new(),
                formatter: MockFormatter,
            }
        }

        fn serialize_seq(self, len: Option<usize>) -> Result<(), std::io::Error> {
            self.formatter.begin_array(&mut self.writer)?;
            if len == Some(0) {
                self.formatter.end_array(&mut self.writer)?;
            }
            Ok(())
        }
    }

    let serializer = Serializer::new();
    let result = serializer.serialize_seq(Some(1));
    assert!(result.is_ok());
}

