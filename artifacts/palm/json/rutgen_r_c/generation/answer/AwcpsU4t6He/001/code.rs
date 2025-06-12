// Answer 0

fn test_begin_array_value_first_true_err() {
    struct DummyWriter {
        should_fail: bool,
    }
    
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_fail {
                Err(io::Error::new(io::ErrorKind::Other, "write failed"))
            } else {
                Ok(buf.len())
            }
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"    ",
    };
    
    let mut writer = DummyWriter {
        should_fail: true,
    };
    
    let result = formatter.begin_array_value(&mut writer, true);
    
    assert!(result.is_err());
}

fn test_begin_array_value_not_first_err() {
    struct DummyWriter {
        should_fail: bool,
    }
    
    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_fail {
                Err(io::Error::new(io::ErrorKind::Other, "write failed"))
            } else {
                Ok(buf.len())
            }
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"    ",
    };
    
    let mut writer = DummyWriter {
        should_fail: true,
    };
    
    let result = formatter.begin_array_value(&mut writer, false);
    
    assert!(result.is_err());
}

