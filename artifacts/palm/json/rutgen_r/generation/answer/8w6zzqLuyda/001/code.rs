// Answer 0

#[test]
fn test_serialize_tuple_variant_err_begin_object() {
    struct MockFormatter;

    impl MockFormatter {
        fn begin_object(&self, _writer: &mut ()) -> Result<(), &'static str> {
            Err("Begin object error")
        }

        fn begin_object_key(&self, _writer: &mut (), _first: bool) -> Result<(), &'static str> {
            Ok(())
        }

        fn end_object_key(&self, _writer: &mut ()) -> Result<(), &'static str> {
            Ok(())
        }

        fn begin_object_value(&self, _writer: &mut ()) -> Result<(), &'static str> {
            Ok(())
        }
    }

    struct SerializeTupleVariant {
        formatter: MockFormatter,
        writer: (),
    }

    impl SerializeTupleVariant {
        fn serialize_str(&self, _variant: &str) -> Result<(), &'static str> {
            Ok(())
        }

        fn serialize_seq(&self, _len: Option<usize>) -> Result<(), &'static str> {
            Ok(())
        }
        
        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<(), &'static str> {
            tri!(self
                .formatter
                .begin_object(&mut self.writer));
            tri!(self
                .formatter
                .begin_object_key(&mut self.writer, true));
            tri!(self.serialize_str(variant));
            tri!(self
                .formatter
                .end_object_key(&mut self.writer));
            tri!(self
                .formatter
                .begin_object_value(&mut self.writer));
            self.serialize_seq(Some(len))
        }
    }

    let serializer = SerializeTupleVariant {
        formatter: MockFormatter,
        writer: (),
    };

    let result = serializer.serialize_tuple_variant("name", 0, "variant", 5);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Begin object error");
}

