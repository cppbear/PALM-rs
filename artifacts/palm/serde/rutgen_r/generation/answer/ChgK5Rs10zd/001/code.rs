// Answer 0

#[test]
#[should_panic]
fn test_end_should_panic() {
    struct VoidStruct;

    impl VoidStruct {
        fn end(self) -> Result<(), String> {
            match self.void {}
        }
    }

    let instance = VoidStruct;
    let _result = instance.end();
}

#[test]
fn test_end_should_return_ok() {
    struct VoidStruct {
        void: !, // never type
    }

    impl VoidStruct {
        fn end(self) -> Result<(), String> {
            match self.void {}
        }
    }

    // This structure cannot be instantiated because it contains a never type.
    let _result = std::panic::catch_unwind(|| {
        let instance = VoidStruct { void: std::mem::unreachable!() };
        instance.end();
    });

    // The `_result` will capture the panic, so we expect a panic in the previous code.
    assert!(_result.is_err());
}

