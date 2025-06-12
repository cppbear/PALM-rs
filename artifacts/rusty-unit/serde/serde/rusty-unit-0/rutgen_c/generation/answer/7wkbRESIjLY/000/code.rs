// Answer 0

#[test]
fn test_end_success() {
    struct MockSerializeMap;
    
    impl SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }
        
        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }
        
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut map = MockSerializeMap;
    let flat_map = FlatMapSerializeMap(&mut map);
    let result = flat_map.end();
    assert!(result.is_ok());
}

#[test]
fn test_end_boundary_condition() {
    struct MockSerializeMapEdge;

    impl SerializeMap for MockSerializeMapEdge {
        type Ok = ();
        type Error = ();

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }
        
        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mut map_edge = MockSerializeMapEdge;
    let flat_map_edge = FlatMapSerializeMap(&mut map_edge);
    let result_edge = flat_map_edge.end();
    assert!(result_edge.is_ok());
}

