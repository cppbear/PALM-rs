// Answer 0

#[test]
fn test_replacen_no_expansion_with_no_matches() {
    struct TestReplacer;

    impl TestReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }
    }

    let regex = Regex::new(r"nonexistent").unwrap();
    let text = b"hello world!";
    let limit = 0;
    let replacer = TestReplacer;

    let result = regex.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_expansion_with_some_matches() {
    struct TestReplacer;

    impl TestReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }
    }

    let regex = Regex::new(r"world").unwrap();
    let text = b"hello world, hello world!";
    let limit = 1;
    let replacer = TestReplacer;

    let result = regex.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Owned(b"hello REPLACED, hello world!".to_vec()));
}

#[test]
fn test_replacen_no_expansion_with_multiple_limits() {
    struct TestReplacer;

    impl TestReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }
    }

    let regex = Regex::new(r"hello").unwrap();
    let text = b"hello world, hello again!";
    let limit = 2; // Replace only two instances
    let replacer = TestReplacer;

    let result = regex.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Owned(b"REPLACED world, REPLACED again!".to_vec()));
}

#[test]
fn test_replacen_no_expansion_limit_zero() {
    struct TestReplacer;

    impl TestReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }
    }

    let regex = Regex::new(r"hello").unwrap();
    let text = b"hello world, hello again!";
    let limit = 0; // Replace all instances
    let replacer = TestReplacer;

    let result = regex.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Owned(b"REPLACED world, REPLACED again!".to_vec()));
}

#[test]
#[should_panic]
fn test_replacen_last_match_out_of_bounds() {
    struct TestReplacer;

    impl TestReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }
    }

    let regex = Regex::new(r"hello").unwrap();
    let text = b"hello"; // Only one match
    let limit = 1; // Limit is 1, but if there's an attempt to use an invalid range, should panic
    let replacer = TestReplacer;

    let _ = regex.replacen(text, limit, replacer);
}

