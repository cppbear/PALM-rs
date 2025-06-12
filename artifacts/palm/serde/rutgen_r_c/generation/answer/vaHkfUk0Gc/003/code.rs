// Answer 0

#[test]
fn test_size_hint_both_none() {
    struct TestError;
    
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestA;
    impl IntoDeserializer<'static, TestError> for TestA {
        type Deserializer = Self;
        fn into_deserializer(self) -> Self {
            self
        }
    }

    struct TestB;
    impl IntoDeserializer<'static, TestError> for TestB {
        type Deserializer = Self;
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let visitor = PairVisitor::<TestA, TestB, TestError>(None, None, PhantomData);
    assert_eq!(visitor.size_hint(), Some(0));
}

#[test]
fn test_size_hint_first_some_second_none() {
    struct TestError;
    
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestA;
    impl IntoDeserializer<'static, TestError> for TestA {
        type Deserializer = Self;
        fn into_deserializer(self) -> Self {
            self
        }
    }

    struct TestB;
    impl IntoDeserializer<'static, TestError> for TestB {
        type Deserializer = Self;
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let visitor = PairVisitor::<TestA, TestB, TestError>(Some(TestA), None, PhantomData);
    assert_eq!(visitor.size_hint(), Some(2));
}

#[test]
fn test_size_hint_first_none_second_some() {
    struct TestError;
    
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }

    struct TestA;
    impl IntoDeserializer<'static, TestError> for TestA {
        type Deserializer = Self;
        fn into_deserializer(self) -> Self {
            self
        }
    }

    struct TestB;
    impl IntoDeserializer<'static, TestError> for TestB {
        type Deserializer = Self;
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let visitor = PairVisitor::<TestA, TestB, TestError>(None, Some(TestB), PhantomData);
    assert_eq!(visitor.size_hint(), Some(1));
}

