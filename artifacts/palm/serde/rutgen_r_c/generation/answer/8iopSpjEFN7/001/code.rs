// Answer 0

#[test]
fn test_end_function_panic() {
    struct TestSerializeTuple;

    impl SerializeTuple for Impossible<TestSerializeTuple, Error> {
        type Ok = TestSerializeTuple;
        type Error = Error;

        fn serialize_element<T>(&mut self, _: &T) -> Result<(), Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<TestSerializeTuple, Error> {
            match self.void {}
        }
    }

    let impossible_instance: Impossible<TestSerializeTuple, Error> = Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: PhantomData,
        error: PhantomData,
    };

    let result = std::panic::catch_unwind(|| {
        impossible_instance.end();
    });

    assert!(result.is_err());
}

