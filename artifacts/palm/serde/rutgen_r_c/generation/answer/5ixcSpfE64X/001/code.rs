// Answer 0

#[test]
fn test_bad_type_boolean() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn is_human_readable(&self) -> bool {
            false
        }
        // Other traits would also return not supported
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: DummySerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Boolean);
    assert_eq!(
        error.to_string(),
        "cannot serialize tagged newtype variant TestType::TestVariant containing Boolean"
    );
}

#[test]
fn test_bad_type_integer() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn is_human_readable(&self) -> bool {
            false
        }
        // Other traits would also return not supported
    }
   
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: DummySerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Integer);
    assert_eq!(
        error.to_string(),
        "cannot serialize tagged newtype variant TestType::TestVariant containing Integer"
    );
}

#[test]
fn test_bad_type_float() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn is_human_readable(&self) -> bool {
            false
        }
        // Other traits would also return not supported
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: DummySerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Float);
    assert_eq!(
        error.to_string(),
        "cannot serialize tagged newtype variant TestType::TestVariant containing Float"
    );
}

#[test]
fn test_bad_type_char() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn is_human_readable(&self) -> bool {
            false
        }
        // Other traits would also return not supported
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: DummySerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Char);
    assert_eq!(
        error.to_string(),
        "cannot serialize tagged newtype variant TestType::TestVariant containing Char"
    );
}

#[test]
fn test_bad_type_string() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn is_human_readable(&self) -> bool {
            false
        }
        // Other traits would also return not supported
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: DummySerializer,
    };
    
    let error = serializer.bad_type(Unsupported::String);
    assert_eq!(
        error.to_string(),
        "cannot serialize tagged newtype variant TestType::TestVariant containing String"
    );
}

#[test]
fn test_bad_type_byte_array() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn is_human_readable(&self) -> bool {
            false
        }
        // Other traits would also return not supported
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: DummySerializer,
    };
    
    let error = serializer.bad_type(Unsupported::ByteArray);
    assert_eq!(
        error.to_string(),
        "cannot serialize tagged newtype variant TestType::TestVariant containing ByteArray"
    );
}

#[test]
fn test_bad_type_optional() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn is_human_readable(&self) -> bool {
            false
        }
        // Other traits would also return not supported
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: DummySerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Optional);
    assert_eq!(
        error.to_string(),
        "cannot serialize tagged newtype variant TestType::TestVariant containing Optional"
    );
}

#[test]
fn test_bad_type_sequence() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn is_human_readable(&self) -> bool {
            false
        }
        // Other traits would also return not supported
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: DummySerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Sequence);
    assert_eq!(
        error.to_string(),
        "cannot serialize tagged newtype variant TestType::TestVariant containing Sequence"
    );
}

#[test]
fn test_bad_type_tuple() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn is_human_readable(&self) -> bool {
            false
        }
        // Other traits would also return not supported
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: DummySerializer,
    };
    
    let error = serializer.bad_type(Unsupported::Tuple);
    assert_eq!(
        error.to_string(),
        "cannot serialize tagged newtype variant TestType::TestVariant containing Tuple"
    );
}

#[test]
fn test_bad_type_tuple_struct() {
    struct DummySerializer;
    impl Serializer for DummySerializer {
        type Ok = ();
        type Error = String;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            Err("Not supported".to_string())
        }
        fn is_human_readable(&self) -> bool {
            false
        }
        // Other traits would also return not supported
    }
    
    let serializer = TaggedSerializer {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "TestTag",
        variant_name: "TestVariantName",
        delegate: DummySerializer,
    };
    
    let error = serializer.bad_type(Unsupported::TupleStruct);
    assert_eq!(
        error.to_string(),
        "cannot serialize tagged newtype variant TestType::TestVariant containing TupleStruct"
    );
}

