// Answer 0

fn test_end_success() -> Result<(), String> {
    struct MockMap;
    
    impl MockMap {
        fn new() -> Self {
            MockMap
        }
        
        fn serialize_value(&self, _content: &Content) -> Result<(), String> {
            Ok(())
        }
        
        fn end(&self) -> Result<i32, String> {
            Ok(42)
        }
    }
    
    struct TestStruct<M> {
        map: M,
        name: String,
        fields: Vec<String>,
    }
    
    impl<M> TestStruct<M> 
    where 
        M: Map,
    {
        fn end(mut self) -> Result<M::Ok, M::Error> {
            self.map.serialize_value(&Content::Struct(self.name, self.fields))?;
            self.map.end()
        }
    }

    trait Map {
        type Ok;
        type Error;

        fn serialize_value(&self, content: &Content) -> Result<Self::Ok, Self::Error>;
        fn end(&self) -> Result<Self::Ok, Self::Error>;
    }

    #[derive(Debug)]
    enum Content {
        Struct(String, Vec<String>),
    }

    let mock_map = MockMap::new();
    let test_struct = TestStruct {
        map: mock_map,
        name: String::from("test_name"),
        fields: vec![String::from("field1"), String::from("field2")],
    };

    let result = test_struct.end();
    assert_eq!(result, Ok(42));
    Ok(())
}

#[test]
fn test_end() {
    test_end_success().unwrap();
}

