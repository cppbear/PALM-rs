// Answer 0

#[test]
fn test_serialize_u16() {
    struct MockSerializer;

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = Impossible<(), ()>;
        type SerializeTuple = Impossible<(), ()>;
        type SerializeTupleStruct = Impossible<(), ()>;
        type SerializeTupleVariant = Impossible<(), ()>;
        type SerializeMap = Impossible<(), ()>;
        type SerializeStruct = Impossible<(), ()>;
        type SerializeStructVariant = Impossible<(), ()>;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
            Err(())
        }
        
        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
            Err(())
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Err(())
        }
        
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
            Err(())
        }
    }

    let serializer = MockSerializer;
    let result = serializer.serialize_u16(42);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_serialize_u16_bad_type() {
    struct TestSerializer {
        was_called: bool,
    }

    impl Serializer for TestSerializer {
        type Ok = ();
        type Error = ();
        type SerializeSeq = Impossible<(), ()>;
        type SerializeTuple = Impossible<(), ()>;
        type SerializeTupleStruct = Impossible<(), ()>;
        type SerializeTupleVariant = Impossible<(), ()>;
        type SerializeMap = Impossible<(), ()>;
        type SerializeStruct = Impossible<(), ()>;
        type SerializeStructVariant = Impossible<(), ()>;

        fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
            self.was_called = true;
            Err(())
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            Err(())
        }
        
        fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
            Err(())
        }
    }

    let mut test_serializer = TestSerializer { was_called: false };
    let result = test_serializer.serialize_u16(255);
    assert_eq!(result.is_err(), true);
    assert!(test_serializer.was_called);
}

