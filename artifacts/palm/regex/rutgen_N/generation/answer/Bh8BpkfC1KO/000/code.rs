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
    assert_eq!(std::mem::size_of_val(&printer), std::mem::size_of::<Printer>());
}

