// Answer 0

fn test_serialize_u128_success() {
    struct MockWriter;
    struct MockFormatter {
        begin_called: bool,
        write_called: bool,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), ()> {
            self.begin_called = true;
            Ok(())
        }
        
        fn write_u128(&mut self, _: &mut MockWriter, _: u128) -> Result<(), ()> {
            self.write_called = true;
            Ok(())
        }
        
        fn end_string(&mut self, _: &mut MockWriter) -> Result<(), ()> {
            if self.begin_called && self.write_called {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }
    
    impl Serializer {
        fn serialize_u128(self, value: u128) -> Result<(), ()> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_u128(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let serializer = Serializer {
        writer: MockWriter,
        formatter: MockFormatter {
            begin_called: false,
            write_called: false,
        },
    };

    let result = serializer.serialize_u128(12345);
    assert!(result.is_ok());
}

fn test_serialize_u128_begin_string_failure() {
    struct MockWriter;
    struct MockFormatter {
        should_fail_begin: bool,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), ()> {
            if self.should_fail_begin {
                Err(())
            } else {
                Ok(())
            }
        }
        
        fn write_u128(&mut self, _: &mut MockWriter, _: u128) -> Result<(), ()> {
            Ok(())
        }
        
        fn end_string(&mut self, _: &mut MockWriter) -> Result<(), ()> {
            Ok(())
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl Serializer {
        fn serialize_u128(self, value: u128) -> Result<(), ()> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_u128(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let serializer = Serializer {
        writer: MockWriter,
        formatter: MockFormatter {
            should_fail_begin: true,
        },
    };

    let result = serializer.serialize_u128(12345);
    assert!(result.is_err());
}

fn test_serialize_u128_write_u128_failure() {
    struct MockWriter;
    struct MockFormatter {
        should_fail_write: bool,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<(), ()> {
            Ok(())
        }
        
        fn write_u128(&mut self, _: &mut MockWriter, _: u128) -> Result<(), ()> {
            if self.should_fail_write {
                Err(())
            } else {
                Ok(())
            }
        }
        
        fn end_string(&mut self, _: &mut MockWriter) -> Result<(), ()> {
            Ok(())
        }
    }

    struct Serializer {
        writer: MockWriter,
        formatter: MockFormatter,
    }

    impl Serializer {
        fn serialize_u128(self, value: u128) -> Result<(), ()> {
            self.formatter.begin_string(&mut self.writer)?;
            self.formatter.write_u128(&mut self.writer, value)?;
            self.formatter.end_string(&mut self.writer)
        }
    }

    let serializer = Serializer {
        writer: MockWriter,
        formatter: MockFormatter {
            should_fail_write: true,
        },
    };

    let result = serializer.serialize_u128(12345);
    assert!(result.is_err());
}

