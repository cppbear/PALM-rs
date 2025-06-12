// Answer 0

#[test]
fn test_build() {
    struct DummyStruct;

    impl DummyStruct {
        fn build(&self) -> Printer {
            Printer {
                _priv: (),
            }
        }
    }

    struct Printer {
        _priv: (),
    }

    let dummy = DummyStruct;
    let printer = dummy.build();

    assert_eq!(std::mem::size_of::<Printer>(), std::mem::size_of::<()>()); // To check the size of the struct
}

