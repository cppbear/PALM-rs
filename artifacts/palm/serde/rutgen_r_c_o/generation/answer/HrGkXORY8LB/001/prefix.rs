// Answer 0

#[test]
fn test_serialize_element_error() {
    struct AlwaysErrorSerializer;

    impl Serialize for AlwaysErrorSerializer {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error>
        where
            S: Serializer,
        {
            Err(S::Error::custom("serialization error"))
        }
    }

    struct TestError;

    impl serde::de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Debug { TestError }
    }

    impl SerializeTuple for SerializeTuple<TestError> {
        type Ok = Content;
        type Error = TestError;
        
        fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let value = tri!(value.serialize(ContentSerializer::<TestError>::new()));
            self.elements.push(value);
            Ok(())
        }

        fn end(self) -> Result<Content, Self::Error> {
            Ok(Content::Seq(self.elements))  
        }
    }

    let mut serializer = SerializeTuple::<TestError> { elements: vec![], error: PhantomData };
    
    let _ = serializer.serialize_element(&AlwaysErrorSerializer);
}

