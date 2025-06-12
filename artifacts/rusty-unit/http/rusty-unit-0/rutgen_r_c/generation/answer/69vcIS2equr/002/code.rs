// Answer 0

#[test]
#[should_panic]
fn test_set_green_when_not_yellow() {
    struct LocalDanger {
        state: Danger,
    }

    impl LocalDanger {
        fn is_yellow(&self) -> bool {
            self.state.is_yellow()
        }

        fn set_green(&mut self) {
            debug_assert!(self.is_yellow());
            self.state = Danger::Green;
        }
    }

    let mut danger = LocalDanger { state: Danger::Green };
    danger.set_green(); // This should panic as `is_yellow()` returns false.
}

#[test]
fn test_set_green_when_yellow() {
    struct LocalDanger {
        state: Danger,
    }

    impl LocalDanger {
        fn is_yellow(&self) -> bool {
            self.state.is_yellow()
        }

        fn set_green(&mut self) {
            debug_assert!(self.is_yellow());
            self.state = Danger::Green;
        }
    }

    let mut danger = LocalDanger { state: Danger::Yellow };
    if danger.is_yellow() {
        danger.set_green(); // This should not panic as `is_yellow()` returns true.
    }
    assert!(matches!(danger.state, Danger::Green));
}

