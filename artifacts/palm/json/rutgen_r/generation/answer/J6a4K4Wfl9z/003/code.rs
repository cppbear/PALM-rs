// Answer 0

#[test]
fn test_serialize_i8_success() {
    struct WriterMock {
        output: Vec<u8>,
    }

    struct FormatterMock<'a> {
        writer: &'a mut WriterMock,
    }

    struct SerMock<'a> {
        formatter: FormatterMock<'a>,
    }

    struct Serializer<'a> {
        ser: SerMock<'a>,
    }

    impl<'a> FormatterMock<'a> {
        fn begin_string(&mut self) -> Result<()> {
            self.writer.output.push(b'{'); // begins a valid string
            Ok(())
        }

        fn write_i8(&mut self, value: i8) -> Result<()> {
            self.writer.output.extend_from_slice(&value.to_le_bytes());
            Ok(())
        }

        fn end_string(&mut self) -> Result<()> {
            self.writer.output.push(b'}'); // ends a valid string
            Ok(())
        }
    }

    let mut writer = WriterMock { output: vec![] };
    let formatter = FormatterMock { writer: &mut writer };
    let ser = SerMock { formatter };
    let serializer = Serializer { ser };

    let result = serializer.serialize_i8(42);

    assert!(result.is_ok());
    assert_eq!(writer.output, vec![b'{', 42, b'}']);
}

#[test]
#[should_panic]
fn test_serialize_i8_begin_string_fail() {
    struct WriterMock {
        error: bool,
    }
    
    struct FormatterMock<'a> {
        writer: &'a mut WriterMock,
    }

    struct SerMock<'a> {
        formatter: FormatterMock<'a>,
    }

    struct Serializer<'a> {
        ser: SerMock<'a>,
    }

    impl<'a> FormatterMock<'a> {
        fn begin_string(&mut self) -> Result<()> {
            if self.writer.error {
                Err(Error::io)
            } else {
                Ok(())
            }
        }

        fn write_i8(&mut self, _: i8) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = WriterMock { error: true }; // induces an error
    let formatter = FormatterMock { writer: &mut writer };
    let ser = SerMock { formatter };
    let serializer = Serializer { ser };

    let _ = serializer.serialize_i8(42); // this will panic due to the error
}

#[test]
#[should_panic]
fn test_serialize_i8_write_i8_fail() {
    struct WriterMock {
        write_error: bool,
    }
    
    struct FormatterMock<'a> {
        writer: &'a mut WriterMock,
    }

    struct SerMock<'a> {
        formatter: FormatterMock<'a>,
    }

    struct Serializer<'a> {
        ser: SerMock<'a>,
    }

    impl<'a> FormatterMock<'a> {
        fn begin_string(&mut self) -> Result<()> {
            Ok(())
        }

        fn write_i8(&mut self, _: i8) -> Result<()> {
            if self.writer.write_error {
                Err(Error::io)
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = WriterMock { write_error: true }; // induces an error
    let formatter = FormatterMock { writer: &mut writer };
    let ser = SerMock { formatter };
    let serializer = Serializer { ser };

    let _ = serializer.serialize_i8(42); // this will panic due to write_i8 error
}

