// Answer 0

#[test]
fn test_replacen_no_expansion_no_matches() {
    struct NoExpansionReplacer;
    
    impl NoExpansionReplacer {
        pub fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACEMENT")
        }
    }
    
    let regex = Regex::new(r"abc").unwrap();
    let text = b"xyz";
    
    let result = regex.replacen(text, 0, NoExpansionReplacer);
    
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_expansion_with_matches() {
    struct NoExpansionReplacer;

    impl NoExpansionReplacer {
        pub fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACEMENT")
        }
    }

    let regex = Regex::new(r"cat").unwrap();
    let text = b"the cat sat on the mat";

    let result = regex.replacen(text, 0, NoExpansionReplacer);
    
    assert_eq!(result, Cow::Owned(b"the REPLACEMENT sat on the mat".to_vec()));
}

#[test]
fn test_replacen_with_captures() {
    struct CaptureReplacer;

    impl CaptureReplacer {
        pub fn replace_append(&self, _cap: &Captures, new: &mut Vec<u8>) {
            new.extend_from_slice(b"REPLACED");
        }
    }

    let regex = Regex::new(r"cat").unwrap();
    let text = b"the cat sat on the mat";

    let result = regex.replacen(text, 1, CaptureReplacer);
    
    assert_eq!(result, Cow::Owned(b"the REPLACED sat on the mat".to_vec()));
}

#[test]
fn test_replacen_with_zero_limit_no_matches() {
    struct NoExpansionReplacer;

    impl NoExpansionReplacer {
        pub fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACEMENT")
        }
    }

    let regex = Regex::new(r"xyz").unwrap();
    let text = b"the cat sat on the mat";

    let result = regex.replacen(text, 0, NoExpansionReplacer);

    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_with_zero_limit_with_matches() {
    struct CaptureReplacer;

    impl CaptureReplacer {
        pub fn replace_append(&self, _cap: &Captures, new: &mut Vec<u8>) {
            new.extend_from_slice(b"REPLACED");
        }
    }

    let regex = Regex::new(r"cat").unwrap();
    let text = b"the cat sat on the cat";

    let result = regex.replacen(text, 0, CaptureReplacer);

    assert_eq!(result, Cow::Owned(b"the REPLACED sat on the REPLACED".to_vec()));
}

