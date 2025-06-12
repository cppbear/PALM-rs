// Answer 0

#[test]
fn test_serialize_tuple_struct_err() {
    struct MySerializer;

    impl MySerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), &'static str> {
            Err("Bad type: Unsupported tuple struct")
        }
        
        fn serialize_tuple_struct(
            self,
            _: &'static str,
            _: usize,
        ) -> Result<(), &'static str> {
            Err(self.bad_type(Unsupported::TupleStruct))
        }
    }
    
    #[derive(Debug)]
    enum Unsupported {
        TupleStruct,
    }

    let serializer = MySerializer;
    let result = serializer.serialize_tuple_struct("TestStruct", 2);
    assert!(result.is_err());
    assert_eq!(result, Err("Bad type: Unsupported tuple struct"));
}

