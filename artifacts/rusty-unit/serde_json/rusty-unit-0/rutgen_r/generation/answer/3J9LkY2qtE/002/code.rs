// Answer 0

#[test]
fn test_serialize_struct_variant_success() {
    struct DummyFormatter;

    impl DummyFormatter {
        fn begin_object(&self, _writer: &mut ()) -> Result<(), ()> {
            Ok(())
        }

        fn begin_object_key(&self, _writer: &mut (), _: bool) -> Result<(), ()> {
            Ok(())
        }

        fn end_object_key(&self, _writer: &mut ()) -> Result<(), ()> {
            Ok(())
        }

        fn begin_object_value(&self, _writer: &mut ()) -> Result<(), ()> {
            Ok(())
        }
    }

    struct DummySerializer {
        formatter: DummyFormatter,
        writer: (),
    }

    impl DummySerializer {
        fn serialize_str(&self, _val: &'static str) -> Result<(), ()> {
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
            self.formatter.begin_object(&mut self.writer).map_err(|_| ())?;
            self.formatter.begin_object_key(&mut self.writer, true).map_err(|_| ())?;
            self.serialize_str(variant)?;
            self.formatter.end_object_key(&mut self.writer).map_err(|_| ())?;
            self.formatter.begin_object_value(&mut self.writer).map_err(|_| ())?;
            self.serialize_map(Some(len)).map_err(|_| ())
        }
    }

    let serializer = DummySerializer {
        formatter: DummyFormatter,
        writer: (),
    };

    let result = serializer.serialize_struct_variant("Test", 0, "Variant", 2);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_struct_variant_begin_object_key_fail() {
    struct DummyFormatter;

    impl DummyFormatter {
        fn begin_object(&self, _writer: &mut ()) -> Result<(), ()> {
            Ok(())
        }

        fn begin_object_key(&self, _writer: &mut (), _: bool) -> Result<(), ()> {
            Err(())
        }

        fn end_object_key(&self, _writer: &mut ()) -> Result<(), ()> {
            Ok(())
        }

        fn begin_object_value(&self, _writer: &mut ()) -> Result<(), ()> {
            Ok(())
        }
    }

    struct DummySerializer {
        formatter: DummyFormatter,
        writer: (),
    }

    impl DummySerializer {
        fn serialize_str(&self, _val: &'static str) -> Result<(), ()> {
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
            self.formatter.begin_object(&mut self.writer).map_err(|_| ())?;
            self.formatter.begin_object_key(&mut self.writer, true).map_err(|_| ())?;
            self.serialize_str(variant)?;
            self.formatter.end_object_key(&mut self.writer).map_err(|_| ())?;
            self.formatter.begin_object_value(&mut self.writer).map_err(|_| ())?;
            self.serialize_map(Some(len)).map_err(|_| ())
        }
    }

    let serializer = DummySerializer {
        formatter: DummyFormatter,
        writer: (),
    };

    let _ = serializer.serialize_struct_variant("Test", 0, "Variant", 2);
}

