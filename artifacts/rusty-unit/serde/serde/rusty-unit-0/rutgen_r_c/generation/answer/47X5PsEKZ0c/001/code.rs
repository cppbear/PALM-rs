// Answer 0

#[test]
fn test_serialize_unit_variant_valid() {
    use std::fmt::Write;

    struct TestSerializer<'a> {
        buffer: &'a mut String,
    }

    impl<'a> serde::Serializer for TestSerializer<'a> {
        type Ok = ();
        type Error = std::fmt::Error;
        type SerializeSeq = Impossible<(), std::fmt::Error>;
        type SerializeTuple = Impossible<(), std::fmt::Error>;
        type SerializeTupleStruct = Impossible<(), std::fmt::Error>;
        type SerializeTupleVariant = Impossible<(), std::fmt::Error>;
        type SerializeMap = Impossible<(), std::fmt::Error>;
        type SerializeStruct = Impossible<(), std::fmt::Error>;
        type SerializeStructVariant = Impossible<(), std::fmt::Error>;

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> std::fmt::Result {
            write!(self.buffer, "{}", variant)
        }

        fn serialize_unit(self) -> std::fmt::Result { Ok(()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> std::fmt::Result { Ok(()) }
        fn serialize_bytes(self, _: &[u8]) -> std::fmt::Result { Ok(()) }
        fn serialize_none(self) -> std::fmt::Result { Ok(()) }
        fn serialize_some<T>(self, _: &T) -> std::fmt::Result { Ok(()) }
        fn serialize_newtype_variant<T>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: &T,
        ) -> std::fmt::Result { Ok(()) }
        fn serialize_seq(self, _: Option<usize>) -> std::result::Result<Self::SerializeSeq, std::fmt::Error> { Err(std::fmt::Error) }
        fn serialize_struct(self, _: &'static str, _: usize) -> std::result::Result<Self::SerializeStruct, std::fmt::Error> { Err(std::fmt::Error) }
    }

    let mut output = String::new();
    let serializer = TestSerializer { buffer: &mut output };

    let result = serializer.serialize_unit_variant("TestName", 0, "Variant");
    
    assert!(result.is_ok());
    assert_eq!(output, "Variant");
}

#[test]
#[should_panic]
fn test_serialize_unit_variant_panic() {
    struct PanickingSerializer;

    impl serde::Serializer for PanickingSerializer {
        type Ok = ();
        type Error = std::fmt::Error;
        type SerializeSeq = Impossible<(), std::fmt::Error>;
        type SerializeTuple = Impossible<(), std::fmt::Error>;
        type SerializeTupleStruct = Impossible<(), std::fmt::Error>;
        type SerializeTupleVariant = Impossible<(), std::fmt::Error>;
        type SerializeMap = Impossible<(), std::fmt::Error>;
        type SerializeStruct = Impossible<(), std::fmt::Error>;
        type SerializeStructVariant = Impossible<(), std::fmt::Error>;

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> std::fmt::Result {
            panic!("This serializer panics!");
        }

        fn serialize_unit(self) -> std::fmt::Result { Ok(()) }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> std::fmt::Result { Ok(()) }
        fn serialize_bytes(self, _: &[u8]) -> std::fmt::Result { Ok(()) }
        fn serialize_none(self) -> std::fmt::Result { Ok(()) }
        fn serialize_some<T>(self, _: &T) -> std::fmt::Result { Ok(()) }
        fn serialize_newtype_variant<T>(
            self,
            _: &'static str,
            _: u32,
            _: &'static str,
            _: &T,
        ) -> std::fmt::Result { Ok(()) }
        fn serialize_seq(self, _: Option<usize>) -> std::result::Result<Self::SerializeSeq, std::fmt::Error> { Err(std::fmt::Error) }
        fn serialize_struct(self, _: &'static str, _: usize) -> std::result::Result<Self::SerializeStruct, std::fmt::Error> { Err(std::fmt::Error) }
    }

    let serializer = PanickingSerializer;
    let _ = serializer.serialize_unit_variant("TestName", 0, "Variant");
}

