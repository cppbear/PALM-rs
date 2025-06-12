// Answer 0

#[test]
fn test_eat_char() {
    struct Reader {
        discarded: bool,
    }

    impl Reader {
        fn discard(&mut self) {
            self.discarded = true;
        }
    }

    struct TestStruct {
        read: Reader,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { read: Reader { discarded: false } }
        }

        fn eat_char(&mut self) {
            self.read.discard();
        }
    }

    let mut test_instance = TestStruct::new();
    test_instance.eat_char();
    assert!(test_instance.read.discarded);
}

#[test]
#[should_panic]
fn test_eat_char_should_panic() {
    struct Reader {
        discarded: bool,
    }

    impl Reader {
        fn discard(&mut self) {
            // Simulate a panic scenario
            panic!("Expected panic scenario.");
        }
    }

    struct TestStruct {
        read: Reader,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct { read: Reader { discarded: false } }
        }

        fn eat_char(&mut self) {
            self.read.discard();
        }
    }

    let mut test_instance = TestStruct::new();
    test_instance.eat_char();
}

