// Answer 0

#[test]
fn test_serialize_i32_success() {
    struct MockWriter;
    impl std::io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0) // Mimic successful write
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        success: bool,
    }

    impl MockFormatter {
        fn begin_string(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            if self.success {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Begin error"))
            }
        }

        fn write_i32(&self, _: &mut dyn std::io::Write, _: i32) -> std::io::Result<()> {
            if self.success {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Write error"))
            }
        }

        fn end_string(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockSer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl MockSer {
        fn serialize_i32(self, value: i32) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_i32(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)?;
            Ok(())
        }
    }

    let ser = MockSer {
        writer: MockWriter,
        formatter: MockFormatter { success: true },
    };
    assert!(ser.serialize_i32(42).is_ok());
}

#[test]
#[should_panic(expected = "Begin error")]
fn test_serialize_i32_begin_error() {
    struct MockWriter;
    impl std::io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        return_error: bool,
    }

    impl MockFormatter {
        fn begin_string(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            if self.return_error {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Begin error"))
            } else {
                Ok(())
            }
        }

        fn write_i32(&self, _: &mut dyn std::io::Write, _: i32) -> std::io::Result<()> {
            Ok(())
        }

        fn end_string(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockSer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl MockSer {
        fn serialize_i32(self, value: i32) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_i32(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)?;
            Ok(())
        }
    }

    let ser = MockSer {
        writer: MockWriter,
        formatter: MockFormatter { return_error: true },
    };
    let _ = ser.serialize_i32(42);
}

#[test]
#[should_panic(expected = "Write error")]
fn test_serialize_i32_write_error() {
    struct MockWriter;
    impl std::io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        return_error: bool,
    }

    impl MockFormatter {
        fn begin_string(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }

        fn write_i32(&self, _: &mut dyn std::io::Write, _: i32) -> std::io::Result<()> {
            if self.return_error {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Write error"))
            } else {
                Ok(())
            }
        }

        fn end_string(&self, _: &mut dyn std::io::Write) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockSer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl MockSer {
        fn serialize_i32(self, value: i32) -> Result<(), std::io::Error> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_i32(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)?;
            Ok(())
        }
    }

    let ser = MockSer {
        writer: MockWriter,
        formatter: MockFormatter { return_error: true },
    };
    let _ = ser.serialize_i32(42);
}

