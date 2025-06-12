// Answer 0

#[test]
fn test_serialize_seq_err() {
    struct MockMap;

    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<K: ?Sized + Serialize>(&mut self, _: &K) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
        
        fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(
            &mut self,
            _: &K,
            _: &V,
        ) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
        
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let mut mock_map = MockMap;
    let serializer = FlatMapSerializer(&mut mock_map);
    
    let result = serializer.serialize_seq(None);
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.err, Error::custom("can only flatten structs and maps (got Sequence)"));
    }
}

