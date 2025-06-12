// Answer 0

#[test]
fn test_serialize_struct_variant() {
    struct MockFormatter {
        should_return_ok: bool,
    }

    impl MockFormatter {
        fn begin_object(&self, _: &mut ()) -> Result<(), ()> {
            if self.should_return_ok {
                Ok(())
            } else {
                Err(())
            }
        }

        fn begin_object_key(&self, _: &mut (), _: bool) -> Result<(), ()> {
            if self.should_return_ok {
                Ok(())
            } else {
                Err(())
            }
        }

        fn end_object_key(&self, _: &mut ()) -> Result<(), ()> {
            if self.should_return_ok {
                Ok(())
            } else {
                Err(())
            }
        }

        fn begin_object_value(&self, _: &mut ()) -> Result<(), ()> {
            if self.should_return_ok {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: (),
    }

    impl MockSerializer {
        fn serialize_str(&self, _: &str) -> Result<(), ()> {
            if self.formatter.should_return_ok {
                Ok(())
            } else {
                Err(())
            }
        }

        fn serialize_map(&self, _: Option<usize>) -> Result<(), ()> {
            if self.formatter.should_return_ok {
                Ok(())
            } else {
                Err(())
            }
        }

        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<(), ()> {
            self.formatter.begin_object(&mut self.writer)?;
            self.formatter.begin_object_key(&mut self.writer, true)?;
            self.serialize_str(variant)?;
            self.formatter.end_object_key(&mut self.writer)?;
            self.formatter.begin_object_value(&mut self.writer)?;
            self.serialize_map(Some(len))?;
            Ok(())
        }
    }

    let serializer = MockSerializer {
        formatter: MockFormatter { should_return_ok: true },
        writer: (),
    };

    let result = serializer.serialize_struct_variant("TestName", 1, "Variant", 5);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_struct_variant_panics_on_formatter_error() {
    struct MockFormatter {
        should_return_ok: bool,
    }

    impl MockFormatter {
        fn begin_object(&self, _: &mut ()) -> Result<(), ()> {
            if self.should_return_ok {
                Ok(())
            } else {
                Err(())
            }
        }

        fn begin_object_key(&self, _: &mut (), _: bool) -> Result<(), ()> {
            Err(())
        }

        fn end_object_key(&self, _: &mut ()) -> Result<(), ()> {
            Ok(())
        }

        fn begin_object_value(&self, _: &mut ()) -> Result<(), ()> {
            Ok(())
        }
    }

    struct MockSerializer {
        formatter: MockFormatter,
        writer: (),
    }

    impl MockSerializer {
        fn serialize_str(&self, _: &str) -> Result<(), ()> {
            Ok(())
        }

        fn serialize_map(&self, _: Option<usize>) -> Result<(), ()> {
            Ok(())
        }

        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<(), ()> {
            self.formatter.begin_object(&mut self.writer)?;
            self.formatter.begin_object_key(&mut self.writer, true)?;
            self.serialize_str(variant)?;
            self.formatter.end_object_key(&mut self.writer)?;
            self.formatter.begin_object_value(&mut self.writer)?;
            self.serialize_map(Some(len))?;
            Ok(())
        }
    }

    let serializer = MockSerializer {
        formatter: MockFormatter { should_return_ok: false },
        writer: (),
    };

    let _ = serializer.serialize_struct_variant("TestName", 1, "Variant", 5);
}

