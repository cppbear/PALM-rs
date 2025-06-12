// Answer 0

#[test]
fn test_serialize_element_bool() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl serde::ser::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestSerializeTuple {
        elements: Vec<Content>,
    }

    impl ser::SerializeTuple for TestSerializeTuple {
        type Ok = Content;
        type Error = TestError;

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let content = Content::Bool(true);
            self.elements.push(content);
            Ok(())
        }

        fn end(self) -> Result<Content, Self::Error> {
            Ok(Content::Seq(self.elements))
        }
    }

    let mut tuple = TestSerializeTuple { elements: Vec::new() };
    assert_eq!(tuple.serialize_element(&true), Ok(()));
}

#[test]
fn test_serialize_element_u8() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl serde::ser::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestSerializeTuple {
        elements: Vec<Content>,
    }

    impl ser::SerializeTuple for TestSerializeTuple {
        type Ok = Content;
        type Error = TestError;

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let content = Content::U8(5);
            self.elements.push(content);
            Ok(())
        }

        fn end(self) -> Result<Content, Self::Error> {
            Ok(Content::Seq(self.elements))
        }
    }

    let mut tuple = TestSerializeTuple { elements: Vec::new() };
    assert_eq!(tuple.serialize_element(&5u8), Ok(()));
}

#[test]
fn test_serialize_element_string() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl serde::ser::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestSerializeTuple {
        elements: Vec<Content>,
    }

    impl ser::SerializeTuple for TestSerializeTuple {
        type Ok = Content;
        type Error = TestError;

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let content = Content::String("test".to_string());
            self.elements.push(content);
            Ok(())
        }

        fn end(self) -> Result<Content, Self::Error> {
            Ok(Content::Seq(self.elements))
        }
    }

    let mut tuple = TestSerializeTuple { elements: Vec::new() };
    assert_eq!(tuple.serialize_element(&"test".to_string()), Ok(()));
}

#[test]
fn test_serialize_element_f32() {
    struct TestError;
    impl std::fmt::Debug for TestError {}
    impl serde::ser::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestSerializeTuple {
        elements: Vec<Content>,
    }

    impl ser::SerializeTuple for TestSerializeTuple {
        type Ok = Content;
        type Error = TestError;

        fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let content = Content::F32(3.14);
            self.elements.push(content);
            Ok(())
        }

        fn end(self) -> Result<Content, Self::Error> {
            Ok(Content::Seq(self.elements))
        }
    }

    let mut tuple = TestSerializeTuple { elements: Vec::new() };
    assert_eq!(tuple.serialize_element(&3.14f32), Ok(()));
}

