// Answer 0

#[test]
fn test_size_hint_both_none() {
    struct TestError;
    impl de::Error for TestError {}
    
    struct TestPairVisitor<E>(Option<i32>, Option<i32>, PhantomData<E>);

    impl<'de, E> de::SeqAccess<'de> for TestPairVisitor<E>
    where
        E: de::Error,
    {
        type Error = E;

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: de::DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn size_hint(&self) -> Option<usize> {
            if self.0.is_some() {
                Some(2)
            } else if self.1.is_some() {
                Some(1)
            } else {
                Some(0)
            }
        }
    }

    let visitor = TestPairVisitor::<TestError>(None, None, PhantomData);
    let _ = visitor.size_hint();
}

