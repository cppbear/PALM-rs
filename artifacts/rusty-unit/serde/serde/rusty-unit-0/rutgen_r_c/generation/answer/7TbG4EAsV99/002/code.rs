// Answer 0

#[test]
fn test_serialize_field_with_bool() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl serde::ser::Error for TestError {
        fn custom<T>(_msg: T) -> Self { TestError }
    }

    struct TestStruct {
        fields: Vec<(&'static str, Content)>,
    }

    impl ser::SerializeStruct for TestStruct {
        type Ok = Content;
        type Error = TestError;

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + serde::Serialize,
        {
            let value = value.serialize(ContentSerializer::<Self::Error>::new()).unwrap();
            self.fields.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, Self::Error> {
            Ok(Content::Struct("TestStruct", self.fields))
        }
    }

    let mut test_struct = TestStruct { fields: Vec::new() };
    let result = test_struct.serialize_field("test_key", &true);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_field_with_u32() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl serde::ser::Error for TestError {
        fn custom<T>(_msg: T) -> Self { TestError }
    }

    struct TestStruct {
        fields: Vec<(&'static str, Content)>,
    }

    impl ser::SerializeStruct for TestStruct {
        type Ok = Content;
        type Error = TestError;

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + serde::Serialize,
        {
            let value = value.serialize(ContentSerializer::<Self::Error>::new()).unwrap();
            self.fields.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, Self::Error> {
            Ok(Content::Struct("TestStruct", self.fields))
        }
    }

    let mut test_struct = TestStruct { fields: Vec::new() };
    let result = test_struct.serialize_field("number_key", &42u32);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_field_with_string() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl serde::ser::Error for TestError {
        fn custom<T>(_msg: T) -> Self { TestError }
    }

    struct TestStruct {
        fields: Vec<(&'static str, Content)>,
    }

    impl ser::SerializeStruct for TestStruct {
        type Ok = Content;
        type Error = TestError;

        fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + serde::Serialize,
        {
            let value = value.serialize(ContentSerializer::<Self::Error>::new()).unwrap();
            self.fields.push((key, value));
            Ok(())
        }

        fn end(self) -> Result<Content, Self::Error> {
            Ok(Content::Struct("TestStruct", self.fields))
        }
    }

    let mut test_struct = TestStruct { fields: Vec::new() };
    let result = test_struct.serialize_field("string_key", &"test string");
    assert!(result.is_ok());
}

