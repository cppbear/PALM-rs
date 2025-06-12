// Answer 0

#[test]
fn test_serialize_tuple_variant_success() {
    struct MockFormatter {
        calls: Vec<String>,
    }

    impl MockFormatter {
        fn begin_object(&mut self, _: &mut ()) -> Result<(), std::io::Error> {
            self.calls.push("begin_object".to_string());
            Ok(())
        }

        fn begin_object_key(&mut self, _: &mut (), _: bool) -> Result<(), std::io::Error> {
            self.calls.push("begin_object_key".to_string());
            Ok(())
        }

        fn end_object_key(&mut self, _: &mut ()) -> Result<(), std::io::Error> {
            self.calls.push("end_object_key".to_string());
            Ok(())
        }

        fn begin_object_value(&mut self, _: &mut ()) -> Result<(), std::io::Error> {
            self.calls.push("begin_object_value".to_string());
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: (),
    }

    impl Serializer {
        fn serialize_str(&mut self, _: &'static str) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn serialize_seq(&mut self, _: Option<usize>) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn serialize_tuple_variant(
            &mut self,
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

    let mut serializer = Serializer {
        formatter: MockFormatter { calls: Vec::new() },
        writer: (),
    };

    let result = serializer.serialize_tuple_variant("test", 0, "variant", 2);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_tuple_variant_fail() {
    struct MockFormatterFail {
        calls: Vec<String>,
    }

    impl MockFormatterFail {
        fn begin_object(&mut self, _: &mut ()) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn begin_object_key(&mut self, _: &mut (), _: bool) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn end_object_key(&mut self, _: &mut ()) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn begin_object_value(&mut self, _: &mut ()) -> Result<(), std::io::Error> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "error"))
        }
    }

    struct SerializerFail {
        formatter: MockFormatterFail,
        writer: (),
    }

    impl SerializerFail {
        fn serialize_str(&mut self, _: &'static str) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn serialize_seq(&mut self, _: Option<usize>) -> Result<(), std::io::Error> {
            Ok(())
        }

        fn serialize_tuple_variant(
            &mut self,
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

    let mut serializer = SerializerFail {
        formatter: MockFormatterFail { calls: Vec::new() },
        writer: (),
    };

    let _ = serializer.serialize_tuple_variant("test", 0, "variant", 2);
}

