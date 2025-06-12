// Answer 0

#[test]
fn test_build_function() {
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
    let result = test_instance.build();
    
    assert_eq!(std::mem::Discriminant(&result), std::mem::Discriminant(&Printer { _priv: () }));
}

