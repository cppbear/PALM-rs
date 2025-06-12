// Answer 0

#[test]
fn test_end_with_valid_inputs() {
    struct TestSerializeTuple;
    impl SerializeTuple for Impossible<(), Error> {
        type Ok = ();
        type Error = Error;
        fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Error> where T: ?Sized + Serialize {}
        fn end(self) -> Result<Self::Ok, Self::Error> {
            match self.void {}
        }
    }

    let impossible_instance = Impossible {
        void: Void,
        ok: PhantomData,
        error: PhantomData,
    };

    let _ = impossible_instance.end();
}

#[test]
#[should_panic]
fn test_end_panics_on_void() {
    struct TestSerializeTuple;
    impl SerializeTuple for Impossible<(), Error> {
        type Ok = ();
        type Error = Error;
        fn serialize_element<T>(&mut self, _value: &T) -> Result<(), Error> where T: ?Sized + Serialize {}
        fn end(self) -> Result<Self::Ok, Self::Error> {
            match self.void {}
        }
    }

    let impossible_instance = Impossible {
        void: Void,
        ok: PhantomData,
        error: PhantomData,
    };

    let _ = impossible_instance.end();
}

