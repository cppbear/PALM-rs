// Answer 0

#[test]
fn test_serialize_struct_variant_success() {
    struct MockFormatter {
        start_called: bool,
        key_called: bool,
        value_called: bool,
    }

    impl MockFormatter {
        fn begin_object(&mut self, _: &mut ()) -> Result<(), ()> {
            self.start_called = true;
            Ok(())
        }

        fn begin_object_key(&mut self, _: &mut (), _: bool) -> Result<(), ()> {
            self.key_called = true;
            Ok(())
        }

        fn end_object_key(&mut self, _: &mut ()) -> Result<(), ()> {
            Ok(())
        }

        fn begin_object_value(&mut self, _: &mut ()) -> Result<(), ()> {
            self.value_called = true;
            Ok(())
        }
    }

    struct Serializer {
        formatter: MockFormatter,
        writer: (),
    }

    impl Serializer {
        fn serialize_str(&self, _: &'static str) -> Result<(), ()> {
            Ok(())
        }
        
        fn serialize_map(&self, _: Option<usize>) -> Result<(), ()> {
            Ok(())
        }

        fn serialize_struct_variant(
            &mut self,
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

    let mut serializer = Serializer {
        formatter: MockFormatter {
            start_called: false,
            key_called: false,
            value_called: false,
        },
        writer: (),
    };

    let result = serializer.serialize_struct_variant("MyStruct", 0, "MyVariant", 2);

    assert!(result.is_ok());
    assert!(serializer.formatter.start_called);
    assert!(serializer.formatter.key_called);
    assert!(serializer.formatter.value_called);
}

#[test]
#[should_panic]
fn test_serialize_struct_variant_failure_on_serialize_str() {
    struct MockFormatter;

    struct Serializer<'a> {
        formatter: &'a MockFormatter,
        writer: (),
    }

    impl<'a> Serializer<'a> {
        fn serialize_str(&self, _: &'static str) -> Result<(), ()> {
            Err(())
        }

        fn serialize_struct_variant(
            &mut self,
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

        fn serialize_map(&self, _: Option<usize>) -> Result<(), ()> {
            Ok(())
        }
    }

    let formatter = MockFormatter;
    let mut serializer = Serializer {
        formatter: &formatter,
        writer: (),
    };

    let _ = serializer.serialize_struct_variant("MyStruct", 0, "MyVariant", 2);
}

