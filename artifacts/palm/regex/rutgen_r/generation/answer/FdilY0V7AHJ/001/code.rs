// Answer 0

#[test]
fn test_only_utf8_enable() {
    struct Compiled {
        only_utf8: bool,
    }

    struct TestStruct {
        compiled: Compiled,
    }

    impl TestStruct {
        fn only_utf8(mut self, yes: bool) -> Self {
            self.compiled.only_utf8 = yes;
            self
        }
    }

    let initial_state = TestStruct {
        compiled: Compiled { only_utf8: false },
    };

    let result = initial_state.only_utf8(true);
    assert_eq!(result.compiled.only_utf8, true);
}

#[test]
fn test_only_utf8_disable() {
    struct Compiled {
        only_utf8: bool,
    }

    struct TestStruct {
        compiled: Compiled,
    }

    impl TestStruct {
        fn only_utf8(mut self, yes: bool) -> Self {
            self.compiled.only_utf8 = yes;
            self
        }
    }

    let initial_state = TestStruct {
        compiled: Compiled { only_utf8: true },
    };

    let result = initial_state.only_utf8(false);
    assert_eq!(result.compiled.only_utf8, false);
}

