// Answer 0

#[test]
fn test_serialize_unit_struct_success() {
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = Impossible<(), ()>;
        type SerializeTuple = Impossible<(), ()>;
        type SerializeTupleStruct = Impossible<(), ()>;
        type SerializeMap = MockSerializeMap;
        type SerializeStruct = Impossible<(), ()>;
        type SerializeTupleVariant = Impossible<(), ()>;
        type SerializeStructVariant = Impossible<(), ()>;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { unimplemented!() }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { unimplemented!() }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unimplemented!() }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { unimplemented!() }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(MockSerializeMap { entries: vec![] })
        }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { unimplemented!() }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { unimplemented!() }
    }

    struct MockSerializeMap {
        entries: Vec<(&'static str, ())>,
    }

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = &'static str;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error> where T: ?Sized + Serialize { Ok(()) }
        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error> where T: ?Sized + Serialize { Ok(()) }
        fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
        where K: ?Sized + Serialize, V: ?Sized + Serialize {
            self.entries.push((key.serialize(&mut *self).unwrap(), ()));
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mock_serializer = TaggedSerializer {
        type_ident: "test_type",
        variant_ident: "test_variant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: MockSerializer,
    };

    let result = mock_serializer.serialize_unit_struct("test_struct");
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_unit_struct_failure() {
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = Impossible<(), ()>;
        type SerializeTuple = Impossible<(), ()>;
        type SerializeTupleStruct = Impossible<(), ()>;
        type SerializeMap = MockSerializeMap;
        type SerializeStruct = Impossible<(), ()>;
        type SerializeTupleVariant = Impossible<(), ()>;
        type SerializeStructVariant = Impossible<(), ()>;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
        fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_newtype_variant<T>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize { unimplemented!() }
        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { unimplemented!() }
        fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> { unimplemented!() }
        fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unimplemented!() }
        fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { unimplemented!() }
        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Ok(MockSerializeMap { entries: vec![] })
        }
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> { unimplemented!() }
        fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> { unimplemented!() }
    }

    struct MockSerializeMap {
        entries: Vec<(&'static str, ())>,
    }

    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = &'static str;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error> where T: ?Sized + Serialize { Ok(()) }
        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error> where T: ?Sized + Serialize { Ok(()) }
        fn serialize_entry<K, V>(&mut self, _: &K, _: &V) -> Result<(), Self::Error>
        where K: ?Sized + Serialize, V: ?Sized + Serialize {
            Err("error");
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mock_serializer = TaggedSerializer {
        type_ident: "test_type",
        variant_ident: "test_variant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: MockSerializer,
    };

    let _ = mock_serializer.serialize_unit_struct("test_struct");
}

