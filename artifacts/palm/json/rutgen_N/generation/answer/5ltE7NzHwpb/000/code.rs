// Answer 0

#[test]
fn test_discard() {
    struct Delegate {
        discarded: bool,
    }

    impl Delegate {
        fn new() -> Self {
            Delegate {
                discarded: false,
            }
        }

        fn discard(&mut self) {
            self.discarded = true;
        }
    }

    struct TestStruct {
        delegate: Delegate,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                delegate: Delegate::new(),
            }
        }

        fn discard(&mut self) {
            self.delegate.discard();
        }
    }

    let mut test_instance = TestStruct::new();
    assert!(!test_instance.delegate.discarded);
    test_instance.discard();
    assert!(test_instance.delegate.discarded);
}

