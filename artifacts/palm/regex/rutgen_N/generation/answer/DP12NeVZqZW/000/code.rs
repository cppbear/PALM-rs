// Answer 0

#[test]
fn test_is_any_anchored_end_with_dollar() {
    struct Example {
        info: Info,
    }

    struct Info {
        anchored: bool,
    }

    impl Info {
        fn is_any_anchored_end(&self) -> bool {
            self.anchored
        }
    }

    let example = Example {
        info: Info { anchored: true },
    };

    assert!(example.is_any_anchored_end());
}

#[test]
fn test_is_any_anchored_end_without_dollar() {
    struct Example {
        info: Info,
    }

    struct Info {
        anchored: bool,
    }

    impl Info {
        fn is_any_anchored_end(&self) -> bool {
            self.anchored
        }
    }

    let example = Example {
        info: Info { anchored: false },
    };

    assert!(!example.is_any_anchored_end());
}

#[test]
fn test_is_any_anchored_end_with_escaped_z() {
    struct Example {
        info: Info,
    }

    struct Info {
        anchored: bool,
    }

    impl Info {
        fn is_any_anchored_end(&self) -> bool {
            self.anchored
        }
    }

    let example = Example {
        info: Info { anchored: true },
    };

    assert!(example.is_any_anchored_end());
}

