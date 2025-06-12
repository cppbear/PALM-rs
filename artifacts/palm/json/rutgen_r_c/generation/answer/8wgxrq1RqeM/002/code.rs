// Answer 0

#[test]
fn test_serialize_neg_int() {
    use serde_json::Serializer;

    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = serde_json::Error;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();
        
        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
            unimplemented!()
        }
        
        fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
            assert_eq!(v, -42);
            Ok(())
        }

        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
            unimplemented!()
        }

        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
            unimplemented!()
        }
    }

    let number = serde_json::Number { n: serde_json::N::NegInt(-42) };
    let serializer = TestSerializer;

    let result = number.serialize(serializer);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_pos_int() {
    use serde_json::Serializer;

    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = serde_json::Error;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
            assert_eq!(v, 42);
            Ok(())
        }

        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
            unimplemented!()
        }

        fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
            unimplemented!()
        }

        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
            unimplemented!()
        }
    }

    let number = serde_json::Number { n: serde_json::N::PosInt(42) };
    let serializer = TestSerializer;

    let result = number.serialize(serializer);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_float() {
    use serde_json::Serializer;

    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = serde_json::Error;
        type SerializeSeq = ();
        type SerializeTuple = ();
        type SerializeTupleStruct = ();
        type SerializeTupleVariant = ();
        type SerializeMap = ();
        type SerializeStruct = ();
        type SerializeStructVariant = ();

        fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
            unimplemented!()
        }
        
        fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
            unimplemented!()
        }

        fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
            assert_eq!(v, 3.14);
            Ok(())
        }

        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
            unimplemented!()
        }
    }

    let number = serde_json::Number { n: serde_json::N::Float(3.14) };
    let serializer = TestSerializer;

    let result = number.serialize(serializer);
    assert!(result.is_ok());
}

