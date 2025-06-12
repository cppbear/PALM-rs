// Answer 0

#[test]
fn test_end_function_panics() {
    struct TestSerializeSeq;

    impl<Ok> SerializeSeq for Impossible<Ok, Error> 
    where 
        Error: ser::Error,
    {
        type Ok = Ok;
        type Error = Error;

        fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Error> 
        where 
            T: ?Sized + Serialize 
        {
            Ok(())
        }

        fn end(self) -> Result<Ok, Error> {
            match self.void {} // should panic
        }
    }

    let impossible_instance: Impossible<(), Error> = Impossible {
        void: Void,
        ok: PhantomData,
        error: PhantomData,
    };
    
    let result: Result<(), Error> = std::panic::catch_unwind(|| {
        impossible_instance.end().unwrap();
    });

    assert!(result.is_err());
}

#[test]
fn test_end_function_return_type() {
    struct TestSerializeSeq;
    
    impl<Ok> SerializeSeq for Impossible<Ok, Error> 
    where 
        Error: ser::Error,
    {
        type Ok = Ok;
        type Error = Error;

        fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Error> 
        where 
            T: ?Sized + Serialize 
        {
            Ok(())
        }

        fn end(self) -> Result<Ok, Error> {
            match self.void {} // should panic
        }
    }

    let impossible_instance: Impossible<(), Error> = Impossible {
        void: Void,
        ok: PhantomData,
        error: PhantomData,
    };

    let result: Result<(), Error> = std::panic::catch_unwind(|| {
        impossible_instance.end().unwrap();
    });

    assert!(result.is_err());
}

