// Answer 0

#[test]
fn test_bad_type_valid_identifier_valid_variant_boolean() {
    let serializer = TaggedSerializer {
        type_ident: "valid_identifier",
        variant_ident: "valid_variant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: DummySerializer {},
    };
    let _ = serializer.bad_type(Unsupported::Boolean);
}

#[test]
fn test_bad_type_long_identifier_empty_variant_integer() {
    let serializer = TaggedSerializer {
        type_ident: "very_long_identifier_exceeding_normal_length",
        variant_ident: "",
        tag: "tag",
        variant_name: "variant_name",
        delegate: DummySerializer {},
    };
    let _ = serializer.bad_type(Unsupported::Integer);
}

#[test]
fn test_bad_type_empty_identifier_empty_variant_float() {
    let serializer = TaggedSerializer {
        type_ident: "",
        variant_ident: "",
        tag: "tag",
        variant_name: "variant_name",
        delegate: DummySerializer {},
    };
    let _ = serializer.bad_type(Unsupported::Float);
}

#[test]
fn test_bad_type_valid_identifier_another_valid_variant_char() {
    let serializer = TaggedSerializer {
        type_ident: "valid_identifier",
        variant_ident: "another_valid_variant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: DummySerializer {},
    };
    let _ = serializer.bad_type(Unsupported::Char);
}

#[test]
fn test_bad_type_valid_identifier_valid_variant_string() {
    let serializer = TaggedSerializer {
        type_ident: "valid_identifier",
        variant_ident: "valid_variant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: DummySerializer {},
    };
    let _ = serializer.bad_type(Unsupported::String);
}

#[test]
fn test_bad_type_valid_identifier_empty_variant_bytearray() {
    let serializer = TaggedSerializer {
        type_ident: "valid_identifier",
        variant_ident: "",
        tag: "tag",
        variant_name: "variant_name",
        delegate: DummySerializer {},
    };
    let _ = serializer.bad_type(Unsupported::ByteArray);
}

#[test]
fn test_bad_type_long_identifier_variant_optional() {
    let serializer = TaggedSerializer {
        type_ident: "very_long_identifier_exceeding_normal_length",
        variant_ident: "valid_variant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: DummySerializer {},
    };
    let _ = serializer.bad_type(Unsupported::Optional);
}

#[test]
fn test_bad_type_valid_identifier_empty_variant_sequence() {
    let serializer = TaggedSerializer {
        type_ident: "valid_identifier",
        variant_ident: "",
        tag: "tag",
        variant_name: "variant_name",
        delegate: DummySerializer {},
    };
    let _ = serializer.bad_type(Unsupported::Sequence);
}

#[test]
fn test_bad_type_empty_identifier_valid_variant_tuple() {
    let serializer = TaggedSerializer {
        type_ident: "",
        variant_ident: "valid_variant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: DummySerializer {},
    };
    let _ = serializer.bad_type(Unsupported::Tuple);
}

#[test]
fn test_bad_type_valid_identifier_another_valid_variant_tuple_struct() {
    let serializer = TaggedSerializer {
        type_ident: "valid_identifier",
        variant_ident: "another_valid_variant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: DummySerializer {},
    };
    let _ = serializer.bad_type(Unsupported::TupleStruct);
}

struct DummySerializer;

impl Serializer for DummySerializer {
    type Ok = ();
    type Error = ();
    type SerializeSeq = ();
    type SerializeTuple = ();
    type SerializeTupleStruct = ();
    type SerializeTupleVariant = ();
    type SerializeMap = ();
    type SerializeStruct = ();
    type SerializeStructVariant = ();
    
    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Ok(()) }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { Ok(()) }
    fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Ok(()) }
    fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { Ok(()) }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { Ok(()) }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { Ok(()) }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Ok(()) }
    fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { Ok(()) }
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Ok(()) }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { Ok(()) }
    fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { Ok(()) }
    fn is_human_readable(&self) -> bool { false }
}

