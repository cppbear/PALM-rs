// Answer 0

#[test]
fn test_serialize_i128_success() {
    struct FormatterMock {
        should_fail_begin: bool,
        should_fail_write: bool,
        output: Vec<u8>,
    }

    struct SerMock {
        formatter: FormatterMock,
        writer: Vec<u8>,
    }

    impl FormatterMock {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            if self.should_fail_begin {
                Err(())
            } else {
                Ok(())
            }
        }

        fn write_i128(&mut self, _writer: &mut Vec<u8>, _value: i128) -> Result<(), ()> {
            if self.should_fail_write {
                Err(())
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            Ok(())
        }
    }

    struct Serializer {
        ser: SerMock,
    }

    impl Serializer {
        fn serialize_i128(&mut self, value: i128) -> Result<(), ()> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i128(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let mut ser = Serializer {
        ser: SerMock {
            formatter: FormatterMock {
                should_fail_begin: false,
                should_fail_write: false,
                output: Vec::new(),
            },
            writer: Vec::new(),
        },
    };
    
    let result = ser.serialize_i128(123456789012345678901234567890i128);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_i128_fail_begin() {
    struct FormatterMock {
        should_fail_begin: bool,
        should_fail_write: bool,
        output: Vec<u8>,
    }

    struct SerMock {
        formatter: FormatterMock,
        writer: Vec<u8>,
    }

    impl FormatterMock {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            if self.should_fail_begin {
                Err(())
            } else {
                Ok(())
            }
        }

        fn write_i128(&mut self, _writer: &mut Vec<u8>, _value: i128) -> Result<(), ()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            Ok(())
        }
    }

    struct Serializer {
        ser: SerMock,
    }

    impl Serializer {
        fn serialize_i128(&mut self, value: i128) -> Result<(), ()> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i128(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let mut ser = Serializer {
        ser: SerMock {
            formatter: FormatterMock {
                should_fail_begin: true,
                should_fail_write: false,
                output: Vec::new(),
            },
            writer: Vec::new(),
        },
    };

    ser.serialize_i128(123456789012345678901234567890i128);
}

#[test]
#[should_panic]
fn test_serialize_i128_fail_write() {
    struct FormatterMock {
        should_fail_begin: bool,
        should_fail_write: bool,
        output: Vec<u8>,
    }

    struct SerMock {
        formatter: FormatterMock,
        writer: Vec<u8>,
    }

    impl FormatterMock {
        fn begin_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            Ok(())
        }

        fn write_i128(&mut self, _writer: &mut Vec<u8>, _value: i128) -> Result<(), ()> {
            if self.should_fail_write {
                Err(())
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self, _writer: &mut Vec<u8>) -> Result<(), ()> {
            Ok(())
        }
    }

    struct Serializer {
        ser: SerMock,
    }

    impl Serializer {
        fn serialize_i128(&mut self, value: i128) -> Result<(), ()> {
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_i128(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let mut ser = Serializer {
        ser: SerMock {
            formatter: FormatterMock {
                should_fail_begin: false,
                should_fail_write: true,
                output: Vec::new(),
            },
            writer: Vec::new(),
        },
    };

    ser.serialize_i128(123456789012345678901234567890i128);
}

