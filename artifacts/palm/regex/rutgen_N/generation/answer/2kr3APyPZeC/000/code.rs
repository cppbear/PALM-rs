// Answer 0

#[test]
fn test_build() {
    struct TestStruct;

    impl TestStruct {
        fn build(&self) -> Printer {
            Printer {
                _priv: (),
            }
        }
    }

    struct Printer {
        _priv: (),
    }

    let test_instance = TestStruct;
    let printer = test_instance.build();

    // Check that we can create a Printer instance
    assert!(std::mem::size_of::<Printer>() > 0);
}

