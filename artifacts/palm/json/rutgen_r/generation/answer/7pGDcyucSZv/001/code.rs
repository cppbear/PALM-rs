// Answer 0

fn end_object_value_test() -> io::Result<()> {
    struct Writer {
        buffer: Vec<u8>,
    }

    impl io::Write for Writer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestStruct {
        has_value: bool,
    }

    impl TestStruct {
        fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            self.has_value = true;
            Ok(())
        }
    }

    let mut writer = Writer {
        buffer: Vec::new()
    };
    
    let mut test_struct = TestStruct {
        has_value: false
    };

    let result = test_struct.end_object_value(&mut writer)?;
    assert!(test_struct.has_value);
    assert_eq!(result, Ok(()));
    
    Ok(())
}

#[test]
fn test_end_object_value() {
    end_object_value_test().unwrap();
}

