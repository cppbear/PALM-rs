// Answer 0

#[test]
fn test_serialize_tuple_variant_success() {
    struct TestFormatter {
        write_called: bool,
    }

    impl TestFormatter {
        fn begin_object(&self, _writer: &mut ()) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn begin_object_key(&self, _writer: &mut (), _first: bool) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn end_object_key(&self, _writer: &mut ()) -> Result<(), std::io::Error> {
            Ok(())
        }
        
        fn begin_object_value(&self, _writer: &mut ()) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct TestSer {
        formatter: TestFormatter,
        writer: (),
    }

    impl TestSer {
        fn serialize_str(&self, _variant: &'static str) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn serialize_seq(&self, _len: Option<usize>) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<(), std::io::Error> {
            self.formatter.begin_object(&mut self.writer)?;
            self.formatter.begin_object_key(&mut self.writer, true)?;
            self.serialize_str(variant)?;
            self.formatter.end_object_key(&mut self.writer)?;
            self.formatter.begin_object_value(&mut self.writer)?;
            self.serialize_seq(Some(len))?;
            Ok(())
        }
    }

    let serializer = TestSer {
        formatter: TestFormatter { write_called: false },
        writer: (),
    };

    let result = serializer.serialize_tuple_variant("test", 0, "variant", 5);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_tuple_variant_fail_begin_object() {
    struct TestFormatter;

    impl TestFormatter {
        fn begin_object(&self, _writer: &mut ()) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "error"))
        }

        fn begin_object_key(&self, _writer: &mut (), _first: bool) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_object_key(&self, _writer: &mut ()) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn begin_object_value(&self, _writer: &mut ()) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct TestSer {
        formatter: TestFormatter,
        writer: (),
    }

    impl TestSer {
        fn serialize_str(&self, _variant: &'static str) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn serialize_seq(&self, _len: Option<usize>) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<(), std::io::Error> {
            self.formatter.begin_object(&mut self.writer)?;
            self.formatter.begin_object_key(&mut self.writer, true)?;
            self.serialize_str(variant)?;
            self.formatter.end_object_key(&mut self.writer)?;
            self.formatter.begin_object_value(&mut self.writer)?;
            self.serialize_seq(Some(len))?;
            Ok(())
        }
    }

    let serializer = TestSer {
        formatter: TestFormatter,
        writer: (),
    };

    let _result = serializer.serialize_tuple_variant("test", 0, "variant", 5);
}

