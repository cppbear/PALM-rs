// Answer 0

#[test]
fn test_deserialize_any_with_err_on_visit_seq() {
    struct TestError;
    impl de::Error for TestError {
        fn custom<T>(_msg: T) -> Self {
            TestError
        }
    }
    
    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            Err(TestError::custom("visit_seq error"))
        }
    }

    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = ();
        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                Some(())
            } else {
                None
            }
        }
    }

    let test_iter = TestIterator { count: 1 };
    let deserializer = SeqDeserializer {
        iter: test_iter.fuse(),
        count: 0,
        marker: PhantomData::<TestError>,
    };

    let _result = deserializer.deserialize_any(TestVisitor);
}

#[test]
fn test_deserialize_any_with_non_empty_iter_and_errors() {
    struct CustomError;
    impl de::Error for CustomError {
        fn custom<T>(_msg: T) -> Self {
            CustomError
        }
    }

    struct FailingVisitor;

    impl<'de> de::Visitor<'de> for FailingVisitor {
        type Value = ();
        fn visit_seq<V>(self, _: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            Err(CustomError::custom("Visitor failed"))
        }
    }

    let failing_iter = std::iter::once(()).chain(std::iter::once(()));
    
    let deserializer_fail = SeqDeserializer {
        iter: failing_iter.fuse(),
        count: 1,
        marker: PhantomData::<CustomError>,
    };

    let _result = deserializer_fail.deserialize_any(FailingVisitor);
}

