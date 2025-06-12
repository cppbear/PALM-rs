// Answer 0

#[test]
fn test_serialize_field_with_bool() {
    struct MockMap {
        data: Vec<(&'static str, Content)>,
    }

    impl SerializeMap for MockMap {
        type Error = ();

        fn serialize_entry(&mut self, key: Content, value: Content) -> Result<(), Self::Error> {
            self.data.push((key.to_string().as_str(), value));
            Ok(())
        }

        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { data: Vec::new() };
    let mut serializer = FlatMapSerializeStructVariantAsMapValue {
        map: &mut map,
        name: "TestStruct",
        fields: Vec::new(),
    };

    let result = serializer.serialize_field("test_bool", &Content::Bool(true));
    assert!(result.is_ok());
    assert_eq!(map.data.len(), 1);
    assert_eq!(map.data[0].0, "test_bool");
    if let Content::Bool(value) = &map.data[0].1 {
        assert_eq!(*value, true);
    } else {
        panic!("Expected Content::Bool");
    }
}

#[test]
fn test_serialize_field_with_string() {
    struct MockMap {
        data: Vec<(&'static str, Content)>,
    }

    impl SerializeMap for MockMap {
        type Error = ();

        fn serialize_entry(&mut self, key: Content, value: Content) -> Result<(), Self::Error> {
            self.data.push((key.to_string().as_str(), value));
            Ok(())
        }

        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { data: Vec::new() };
    let mut serializer = FlatMapSerializeStructVariantAsMapValue {
        map: &mut map,
        name: "TestStruct",
        fields: Vec::new(),
    };

    let result = serializer.serialize_field("test_string", &Content::String("hello".to_string()));
    assert!(result.is_ok());
    assert_eq!(map.data.len(), 1);
    assert_eq!(map.data[0].0, "test_string");
    if let Content::String(ref value) = &map.data[0].1 {
        assert_eq!(value, "hello");
    } else {
        panic!("Expected Content::String");
    }
}

#[test]
fn test_serialize_field_with_u32() {
    struct MockMap {
        data: Vec<(&'static str, Content)>,
    }

    impl SerializeMap for MockMap {
        type Error = ();

        fn serialize_entry(&mut self, key: Content, value: Content) -> Result<(), Self::Error> {
            self.data.push((key.to_string().as_str(), value));
            Ok(())
        }

        fn end(self) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    let mut map = MockMap { data: Vec::new() };
    let mut serializer = FlatMapSerializeStructVariantAsMapValue {
        map: &mut map,
        name: "TestStruct",
        fields: Vec::new(),
    };

    let result = serializer.serialize_field("test_u32", &Content::U32(42));
    assert!(result.is_ok());
    assert_eq!(map.data.len(), 1);
    assert_eq!(map.data[0].0, "test_u32");
    if let Content::U32(value) = &map.data[0].1 {
        assert_eq!(*value, 42);
    } else {
        panic!("Expected Content::U32");
    }
}

