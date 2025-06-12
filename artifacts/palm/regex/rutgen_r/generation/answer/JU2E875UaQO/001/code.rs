// Answer 0

#[test]
fn test_replace_all_with_empty_string() {
    struct MockReplacer;
    
    impl Replacer for MockReplacer {
        fn replace(&self, cap: &Captures) -> String {
            String::from("replacement")
        }
    }

    let text = "";
    let replacer = MockReplacer;
    assert_eq!(replace_all(text, replacer), "replacement");
}

#[test]
fn test_replace_all_with_no_matches() {
    struct MockReplacer;

    impl Replacer for MockReplacer {
        fn replace(&self, cap: &Captures) -> String {
            String::from("replacement")
        }
    }

    let text = "this string has no matches";
    let replacer = MockReplacer;
    assert_eq!(replace_all(text, replacer), "this string has no matches");
}

#[test]
fn test_replace_all_with_multiple_matches() {
    struct MockReplacer;

    impl Replacer for MockReplacer {
        fn replace(&self, cap: &Captures) -> String {
            String::from("replacement")
        }
    }

    let text = "replace this and also replace that";
    let replacer = MockReplacer;
    assert_eq!(replace_all(text, replacer), "replacement replacement");
}

#[test]
fn test_replace_all_with_special_characters() {
    struct MockReplacer;

    impl Replacer for MockReplacer {
        fn replace(&self, cap: &Captures) -> String {
            String::from("!@#$%^&*()")
        }
    }

    let text = "replace this $ and that %";
    let replacer = MockReplacer;
    assert_eq!(replace_all(text, replacer), "!@#$%^&*() !@#$%^&*()");
}

#[test]
fn test_replace_all_with_boundary_case() {
    struct MockReplacer;

    impl Replacer for MockReplacer {
        fn replace(&self, cap: &Captures) -> String {
            String::from("#")
        }
    }

    let text = "no matches here";
    let replacer = MockReplacer;
    assert_eq!(replace_all(text, replacer), "no matches here");
}

