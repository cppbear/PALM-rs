// Answer 0

#[test]
fn test_serialize_f64_io_error_begin_string() {
    struct MockFormatter {
        begin_string_result: Result<(), ()>,
        write_f64_result: Result<(), ()>,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut ()) -> Result<(), ()> {
            self.begin_string_result.clone()
        }

        fn write_f64(&mut self, _: &mut (), _: f64) -> Result<(), ()> {
            self.write_f64_result.clone()
        }

        fn end_string(&mut self, _: &mut ()) -> Result<(), ()> {
            Ok(())
        }
    }

    struct MockSerializer {
        writer: (),
        formatter: MockFormatter,
    }

    struct TestStruct {
        ser: MockSerializer,
    }

    impl TestStruct {
        fn serialize_f64(self, value: f64) -> Result<(), ()> {
            if !value.is_finite() {
                return Err(());
            }
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_f64(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let formatter = MockFormatter {
        begin_string_result: Err(()),
        write_f64_result: Ok(()),
    };
    let serializer = MockSerializer {
        writer: (),
        formatter,
    };
    let test_struct = TestStruct { ser: serializer };

    assert_eq!(test_struct.serialize_f64(42.0), Err(()));
}

#[test]
fn test_serialize_f64_io_error_write_f64() {
    struct MockFormatter {
        begin_string_result: Result<(), ()>,
        write_f64_result: Result<(), ()>,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut ()) -> Result<(), ()> {
            self.begin_string_result.clone()
        }

        fn write_f64(&mut self, _: &mut (), _: f64) -> Result<(), ()> {
            self.write_f64_result.clone()
        }

        fn end_string(&mut self, _: &mut ()) -> Result<(), ()> {
            Ok(())
        }
    }

    struct MockSerializer {
        writer: (),
        formatter: MockFormatter,
    }

    struct TestStruct {
        ser: MockSerializer,
    }

    impl TestStruct {
        fn serialize_f64(self, value: f64) -> Result<(), ()> {
            if !value.is_finite() {
                return Err(());
            }
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_f64(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let formatter = MockFormatter {
        begin_string_result: Ok(()),
        write_f64_result: Err(()),
    };
    let serializer = MockSerializer {
        writer: (),
        formatter,
    };
    let test_struct = TestStruct { ser: serializer };

    assert_eq!(test_struct.serialize_f64(42.0), Err(()));
}

#[test]
fn test_serialize_f64_io_error_end_string() {
    struct MockFormatter {
        begin_string_result: Result<(), ()>,
        write_f64_result: Result<(), ()>,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut ()) -> Result<(), ()> {
            self.begin_string_result.clone()
        }

        fn write_f64(&mut self, _: &mut (), _: f64) -> Result<(), ()> {
            self.write_f64_result.clone()
        }

        fn end_string(&mut self, _: &mut ()) -> Result<(), ()> {
            Err(())
        }
    }

    struct MockSerializer {
        writer: (),
        formatter: MockFormatter,
    }

    struct TestStruct {
        ser: MockSerializer,
    }

    impl TestStruct {
        fn serialize_f64(self, value: f64) -> Result<(), ()> {
            if !value.is_finite() {
                return Err(());
            }
            self.ser.formatter.begin_string(&mut self.ser.writer)?;
            self.ser.formatter.write_f64(&mut self.ser.writer, value)?;
            self.ser.formatter.end_string(&mut self.ser.writer)?;
            Ok(())
        }
    }

    let formatter = MockFormatter {
        begin_string_result: Ok(()),
        write_f64_result: Ok(()),
    };
    let serializer = MockSerializer {
        writer: (),
        formatter,
    };
    let test_struct = TestStruct { ser: serializer };

    assert_eq!(test_struct.serialize_f64(42.0), Err(()));
}

