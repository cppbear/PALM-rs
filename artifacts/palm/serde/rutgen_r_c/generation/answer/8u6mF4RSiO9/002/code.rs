// Answer 0

#[test]
fn test_serialize_field_with_bool() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl ser::Error for TestError {}

    struct TestStruct {
        fields: Vec<(&'static str, Content)>,
    }

    impl ser::SerializeStructVariant for TestStruct {
        type Ok = Content;
        type Error = TestError;
        
        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let value = value.serialize(ContentSerializer::new())?;
            self.fields.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, Self::Error> {
            Ok(Content::Struct("TestStruct", self.fields))
        }
    }

    let mut test_struct = TestStruct { fields: vec![] };
    let result = test_struct.serialize_field("bool_field", &true);
    assert_eq!(result, Ok(()));
    assert_eq!(test_struct.fields.len(), 1);
}

#[test]
fn test_serialize_field_with_u8() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl ser::Error for TestError {}

    struct TestStruct {
        fields: Vec<(&'static str, Content)>,
    }

    impl ser::SerializeStructVariant for TestStruct {
        type Ok = Content;
        type Error = TestError;

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let value = value.serialize(ContentSerializer::new())?;
            self.fields.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, Self::Error> {
            Ok(Content::Struct("TestStruct", self.fields))
        }
    }

    let mut test_struct = TestStruct { fields: vec![] };
    let result = test_struct.serialize_field("u8_field", &5u8);
    assert_eq!(result, Ok(()));
    assert_eq!(test_struct.fields.len(), 1);
}

#[test]
fn test_serialize_field_with_string() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl ser::Error for TestError {}

    struct TestStruct {
        fields: Vec<(&'static str, Content)>,
    }

    impl ser::SerializeStructVariant for TestStruct {
        type Ok = Content;
        type Error = TestError;

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let value = value.serialize(ContentSerializer::new())?;
            self.fields.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, Self::Error> {
            Ok(Content::Struct("TestStruct", self.fields))
        }
    }

    let mut test_struct = TestStruct { fields: vec![] };
    let result = test_struct.serialize_field("string_field", &String::from("Hello"));
    assert_eq!(result, Ok(()));
    assert_eq!(test_struct.fields.len(), 1);
}

