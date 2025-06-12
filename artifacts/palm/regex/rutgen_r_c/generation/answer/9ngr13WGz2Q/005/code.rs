// Answer 0

#[test]
fn test_replacen_no_expansion_empty_text() {
    struct DummyReplacer {
        replacement: Vec<u8>,
    }

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(&self.replacement)
        }
    }

    let regex = Regex::new("abc").unwrap();
    let text: &[u8] = b"";
    let replacer = DummyReplacer {
        replacement: b"xyz".to_vec(),
    };
    
    let result = regex.replacen(text, 1, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_expansion_one_match() {
    struct DummyReplacer {
        replacement: Vec<u8>,
    }

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(&self.replacement)
        }
    }

    let regex = Regex::new("abc").unwrap();
    let text: &[u8] = b"abc def abc";
    let replacer = DummyReplacer {
        replacement: b"xyz".to_vec(),
    };

    let result = regex.replacen(text, 1, replacer);
    assert_eq!(result, Cow::Owned(b"xyz def abc".to_vec()));
}

#[test]
fn test_replacen_no_expansion_multiple_matches_with_limit() {
    struct DummyReplacer {
        replacement: Vec<u8>,
    }

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(&self.replacement)
        }
    }

    let regex = Regex::new("abc").unwrap();
    let text: &[u8] = b"abc def abc ghi abc";
    let replacer = DummyReplacer {
        replacement: b"xyz".to_vec(),
    };

    let result = regex.replacen(text, 2, replacer);
    assert_eq!(result, Cow::Owned(b"xyz def xyz ghi abc".to_vec()));
}

#[test]
fn test_replacen_no_expansion_all_matches() {
    struct DummyReplacer {
        replacement: Vec<u8>,
    }

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(&self.replacement)
        }
    }

    let regex = Regex::new("abc").unwrap();
    let text: &[u8] = b"abc def abc ghi abc";
    let replacer = DummyReplacer {
        replacement: b"xyz".to_vec(),
    };

    let result = regex.replacen(text, 0, replacer);
    assert_eq!(result, Cow::Owned(b"xyz def xyz ghi xyz".to_vec()));
}

#[test]
#[should_panic]
fn test_replacen_no_expansion_panic() {
    let regex = Regex::new("abc").unwrap();
    let text: &[u8] = b"abc";
    let replacer = DummyReplacer {
        replacement: b"xyz".to_vec(),
    };

    // this will panic as last_match will be out of range
    let _ = regex.replacen(text, 1, replacer);
}

