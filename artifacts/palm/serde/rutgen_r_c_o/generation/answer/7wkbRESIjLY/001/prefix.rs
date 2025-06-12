// Answer 0

#[test]
fn test_end_success() {
    struct TestMap;
    
    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
            where T: ?Sized + Serialize {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
            where T: ?Sized + Serialize {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = TestMap;
    let flat_map = FlatMapSerializeMap(&mut map);
    let result = flat_map.end();
}

#[test]
fn test_end_with_non_empty_map() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    struct HashMapSerializeMap<'a>(&'a mut HashMap<&'static str, &'static str>);
    
    impl<'a> SerializeMap for HashMapSerializeMap<'a> {
        type Ok = ();
        type Error = ();
        
        fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
            where T: ?Sized + Serialize {
            Ok(())
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
            where T: ?Sized + Serialize {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mut serialize_map = HashMapSerializeMap(&mut map);
    let flat_map = FlatMapSerializeMap(&mut serialize_map);
    let result = flat_map.end();
}

