// Answer 0

#[test]
fn test_serialize_u16_begin_string_err() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut ()) -> Result<()> {
            if self.should_fail {
                Err(Error)
            } else {
                Ok(())
            }
        }

        fn write_u16(&mut self, _: &mut (), _: u16) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut ()) -> Result<()> {
            Ok(())
        }
    }

    struct MockWriter;

    struct MockSerializer<'a> {
        writer: &'a mut MockWriter,
        formatter: MockFormatter,
    }

    let mut writer = MockWriter;
    let serializer = MockSerializer {
        writer: &mut writer,
        formatter: MockFormatter { should_fail: true },
    };

    let key_serializer = MapKeySerializer { ser: &serializer };

    let result = key_serializer.serialize_u16(500);
}

#[test]
fn test_serialize_u16_write_u16_err() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut ()) -> Result<()> {
            Ok(())
        }

        fn write_u16(&mut self, _: &mut (), _: u16) -> Result<()> {
            if self.should_fail {
                Err(Error)
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self, _: &mut ()) -> Result<()> {
            Ok(())
        }
    }

    struct MockWriter;

    struct MockSerializer<'a> {
        writer: &'a mut MockWriter,
        formatter: MockFormatter,
    }

    let mut writer = MockWriter;
    let serializer = MockSerializer {
        writer: &mut writer,
        formatter: MockFormatter { should_fail: true },
    };

    let key_serializer = MapKeySerializer { ser: &serializer };

    let result = key_serializer.serialize_u16(500);
}

#[test]
fn test_serialize_u16_end_string_err() {
    struct MockFormatter {
        should_fail: bool,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut ()) -> Result<()> {
            Ok(())
        }

        fn write_u16(&mut self, _: &mut (), _: u16) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut ()) -> Result<()> {
            if self.should_fail {
                Err(Error)
            } else {
                Ok(())
            }
        }
    }

    struct MockWriter;

    struct MockSerializer<'a> {
        writer: &'a mut MockWriter,
        formatter: MockFormatter,
    }

    let mut writer = MockWriter;
    let serializer = MockSerializer {
        writer: &mut writer,
        formatter: MockFormatter { should_fail: true },
    };

    let key_serializer = MapKeySerializer { ser: &serializer };

    let result = key_serializer.serialize_u16(500);
}

