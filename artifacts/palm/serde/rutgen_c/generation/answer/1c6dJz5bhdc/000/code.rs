// Answer 0

#[test]
fn test_bad_type_with_boolean() {
    struct MockError;
    impl serde::ser::Error for MockError {
        fn custom<T>(_msg: T) -> Self { MockError }
    }
    
    struct MockSerializeMap;
    impl serde::ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = MockError;

        fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error> where K: serde::ser::Serialize, V: serde::ser::Serialize { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let what = Unsupported::Boolean;
    let result = FlatMapSerializer::<MockSerializeMap>(&mut MockSerializeMap).bad_type(what);
    assert_eq!(result.ok(), None);
}

#[test]
fn test_bad_type_with_integer() {
    struct MockError;
    impl serde::ser::Error for MockError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self { MockError }
    }
    
    struct MockSerializeMap;
    impl serde::ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = MockError;

        fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error> where K: serde::ser::Serialize, V: serde::ser::Serialize { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let what = Unsupported::Integer;
    let result = FlatMapSerializer::<MockSerializeMap>(&mut MockSerializeMap).bad_type(what);
    assert_eq!(result.ok(), None);
}

#[test]
fn test_bad_type_with_float() {
    struct MockError;
    impl serde::ser::Error for MockError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self { MockError }
    }
    
    struct MockSerializeMap;
    impl serde::ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = MockError;

        fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error> where K: serde::ser::Serialize, V: serde::ser::Serialize { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let what = Unsupported::Float;
    let result = FlatMapSerializer::<MockSerializeMap>(&mut MockSerializeMap).bad_type(what);
    assert_eq!(result.ok(), None);
}

#[test]
fn test_bad_type_with_char() {
    struct MockError;
    impl serde::ser::Error for MockError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self { MockError }
    }
    
    struct MockSerializeMap;
    impl serde::ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = MockError;

        fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error> where K: serde::ser::Serialize, V: serde::ser::Serialize { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let what = Unsupported::Char;
    let result = FlatMapSerializer::<MockSerializeMap>(&mut MockSerializeMap).bad_type(what);
    assert_eq!(result.ok(), None);
}

#[test]
fn test_bad_type_with_string() {
    struct MockError;
    impl serde::ser::Error for MockError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self { MockError }
    }
    
    struct MockSerializeMap;
    impl serde::ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = MockError;

        fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error> where K: serde::ser::Serialize, V: serde::ser::Serialize { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let what = Unsupported::String;
    let result = FlatMapSerializer::<MockSerializeMap>(&mut MockSerializeMap).bad_type(what);
    assert_eq!(result.ok(), None);
}

#[test]
fn test_bad_type_with_byte_array() {
    struct MockError;
    impl serde::ser::Error for MockError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self { MockError }
    }
    
    struct MockSerializeMap;
    impl serde::ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = MockError;

        fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error> where K: serde::ser::Serialize, V: serde::ser::Serialize { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let what = Unsupported::ByteArray;
    let result = FlatMapSerializer::<MockSerializeMap>(&mut MockSerializeMap).bad_type(what);
    assert_eq!(result.ok(), None);
}

#[test]
fn test_bad_type_with_optional() {
    struct MockError;
    impl serde::ser::Error for MockError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self { MockError }
    }
    
    struct MockSerializeMap;
    impl serde::ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = MockError;

        fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error> where K: serde::ser::Serialize, V: serde::ser::Serialize { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let what = Unsupported::Optional;
    let result = FlatMapSerializer::<MockSerializeMap>(&mut MockSerializeMap).bad_type(what);
    assert_eq!(result.ok(), None);
}

#[test]
fn test_bad_type_with_sequence() {
    struct MockError;
    impl serde::ser::Error for MockError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self { MockError }
    }
    
    struct MockSerializeMap;
    impl serde::ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = MockError;

        fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error> where K: serde::ser::Serialize, V: serde::ser::Serialize { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let what = Unsupported::Sequence;
    let result = FlatMapSerializer::<MockSerializeMap>(&mut MockSerializeMap).bad_type(what);
    assert_eq!(result.ok(), None);
}

#[test]
fn test_bad_type_with_tuple() {
    struct MockError;
    impl serde::ser::Error for MockError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self { MockError }
    }
    
    struct MockSerializeMap;
    impl serde::ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = MockError;

        fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error> where K: serde::ser::Serialize, V: serde::ser::Serialize { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let what = Unsupported::Tuple;
    let result = FlatMapSerializer::<MockSerializeMap>(&mut MockSerializeMap).bad_type(what);
    assert_eq!(result.ok(), None);
}

#[test]
fn test_bad_type_with_tuple_struct() {
    struct MockError;
    impl serde::ser::Error for MockError {
        fn custom<T: std::fmt::Display>(_msg: T) -> Self { MockError }
    }
    
    struct MockSerializeMap;
    impl serde::ser::SerializeMap for MockSerializeMap {
        type Ok = ();
        type Error = MockError;

        fn serialize_entry<K, V>(&mut self, _key: &K, _value: &V) -> Result<(), Self::Error> where K: serde::ser::Serialize, V: serde::ser::Serialize { Ok(()) }
        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let what = Unsupported::TupleStruct;
    let result = FlatMapSerializer::<MockSerializeMap>(&mut MockSerializeMap).bad_type(what);
    assert_eq!(result.ok(), None);
}

