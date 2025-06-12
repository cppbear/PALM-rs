// Answer 0

#[test]
fn test_expand_str_with_number_reference() {
    use re_unicode::{Captures, Ref};

    struct TestCaptures {
        group: String,
    }

    impl TestCaptures {
        fn new() -> Self {
            Self {
                group: "test".to_string(),
            }
        }
    }

    impl Captures for TestCaptures {
        fn get(&self, index: usize) -> Option<&str> {
            if index == 0 { Some(&self.group) } else { None }
        }

        fn name(&self, _: &str) -> Option<&str> {
            None
        }
    }

    let caps = TestCaptures::new();
    let mut dst = String::new();
    let replacement = "$0 and some more text";
    expand_str(&caps, replacement, &mut dst);
    assert_eq!(dst, "test and some more text");
}

#[test]
fn test_expand_str_with_named_reference() {
    use re_unicode::{Captures, Ref};

    struct TestCaptures {
        named: String,
    }

    impl TestCaptures {
        fn new() -> Self {
            Self {
                named: "example".to_string(),
            }
        }
    }

    impl Captures for TestCaptures {
        fn get(&self, _: usize) -> Option<&str> {
            None
        }

        fn name(&self, name: &str) -> Option<&str> {
            if name == "named" { Some(&self.named) } else { None }
        }
    }

    fn find_cap_ref(replacement: &str) -> Option<&'static str> {
        if replacement.starts_with("$named") {
            Some("named")
        } else {
            None
        }
    }

    let caps = TestCaptures::new();
    let mut dst = String::new();
    let replacement = "$named and additional text";
    expand_str(&caps, replacement, &mut dst);
    assert_eq!(dst, "example and additional text");
}

#[test]
fn test_expand_str_with_double_dollar_sign() {
    use re_unicode::{Captures, Ref};

    struct TestCaptures {}

    impl Captures for TestCaptures {
        fn get(&self, _: usize) -> Option<&str> { None }
        fn name(&self, _: &str) -> Option<&str> { None }
    }

    let caps = TestCaptures {};
    let mut dst = String::new();
    let replacement = "$$ and text following";
    expand_str(&caps, replacement, &mut dst);
    assert_eq!(dst, "$ and text following");
}

