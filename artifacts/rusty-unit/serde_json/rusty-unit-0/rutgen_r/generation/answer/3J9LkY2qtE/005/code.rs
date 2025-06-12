// Answer 0

#[test]
fn test_serialize_struct_variant_success() {
    struct DummyFormatter;
    struct DummyWriter;

    impl DummyFormatter {
        fn begin_object(&self, _writer: &mut DummyWriter) -> Result<(), ()> {
            Ok(())
        }
        fn begin_object_key(&self, _writer: &mut DummyWriter, _is_first: bool) -> Result<(), ()> {
            Ok(())
        }
        fn end_object_key(&self, _writer: &mut DummyWriter) -> Result<(), ()> {
            Ok(())
        }
        fn begin_object_value(&self, _writer: &mut DummyWriter) -> Result<(), ()> {
            Ok(())
        }
    }

    struct Serializer<'a> {
        formatter: &'a DummyFormatter,
        writer: DummyWriter,
    }

    impl<'a> Serializer<'a> {
        fn serialize_str(&self, _variant: &'static str) -> Result<(), ()> {
            Ok(())
        }

        fn serialize_map(&self, _len: Option<usize>) -> Result<(), ()> {
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
            self.serialize_map(Some(len))
        }
    }

    let formatter = DummyFormatter;
    let serializer = Serializer {
        formatter: &formatter,
        writer: DummyWriter,
    };

    let result = serializer.serialize_struct_variant("TestName", 0, "TestVariant", 2);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_struct_variant_begin_object_value_fail() {
    struct DummyFormatter;
    struct DummyWriter;

    impl DummyFormatter {
        fn begin_object(&self, _writer: &mut DummyWriter) -> Result<(), ()> {
            Ok(())
        }
        fn begin_object_key(&self, _writer: &mut DummyWriter, _is_first: bool) -> Result<(), ()> {
            Ok(())
        }
        fn end_object_key(&self, _writer: &mut DummyWriter) -> Result<(), ()> {
            Ok(())
        }
        fn begin_object_value(&self, _writer: &mut DummyWriter) -> Result<(), ()> {
            Err(())
        }
    }

    struct Serializer<'a> {
        formatter: &'a DummyFormatter,
        writer: DummyWriter,
    }

    impl<'a> Serializer<'a> {
        fn serialize_str(&self, _variant: &'static str) -> Result<(), ()> {
            Ok(())
        }

        fn serialize_map(&self, _len: Option<usize>) -> Result<(), ()> {
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
            self.serialize_map(Some(len))
        }
    }

    let formatter = DummyFormatter;
    let serializer = Serializer {
        formatter: &formatter,
        writer: DummyWriter,
    };

    let _ = serializer.serialize_struct_variant("TestName", 0, "TestVariant", 2);
}

