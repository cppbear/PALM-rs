// Answer 0

#[test]
fn test_serialize_map_success() {
    struct TestSerializer(usize);
    
    struct FlatMapSerializeMap(usize);

    impl TestSerializer {
        fn serialize_map(self, _: Option<usize>) -> Result<FlatMapSerializeMap, &'static str> {
            Ok(FlatMapSerializeMap(self.0))
        }
    }
    
    let serializer = TestSerializer(42);
    let result = serializer.serialize_map(None);
    
    assert_eq!(result, Ok(FlatMapSerializeMap(42)));
}

#[test]
fn test_serialize_map_with_option() {
    struct TestSerializer(usize);
    
    struct FlatMapSerializeMap(usize);
    
    impl TestSerializer {
        fn serialize_map(self, _: Option<usize>) -> Result<FlatMapSerializeMap, &'static str> {
            Ok(FlatMapSerializeMap(self.0))
        }
    }
    
    let serializer = TestSerializer(100);
    let result = serializer.serialize_map(Some(5));
    
    assert_eq!(result, Ok(FlatMapSerializeMap(100)));
}

