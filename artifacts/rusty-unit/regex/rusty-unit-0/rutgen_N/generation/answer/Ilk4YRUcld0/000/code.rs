// Answer 0

#[test]
fn test_expand_with_valid_integer_capture_group() {
    struct CaptureGroup {
        // A representation of named and indexed capture groups
        groups: Vec<Option<Vec<u8>>>,
    }

    impl CaptureGroup {
        fn new() -> Self {
            Self { groups: vec![Some(b"match".to_vec()), Some(b"group1".to_vec()), None] }
        }

        fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>) {
            expand_bytes(self, replacement, dst)
        }
    }

    let capture_group = CaptureGroup::new();
    let mut dst = Vec::new();
    capture_group.expand(b"$1 and $0", &mut dst);
    assert_eq!(dst, b"group1 and match");
}

#[test]
fn test_expand_with_named_capture_group() {
    struct CaptureGroup {
        groups: std::collections::HashMap<String, Vec<u8>>,
    }

    impl CaptureGroup {
        fn new() -> Self {
            let mut groups = std::collections::HashMap::new();
            groups.insert("name".to_string(), b"named_group".to_vec());
            Self { groups }
        }

        fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>) {
            expand_bytes(self, replacement, dst)
        }
    }

    let capture_group = CaptureGroup::new();
    let mut dst = Vec::new();
    capture_group.expand(b"${name} is here", &mut dst);
    assert_eq!(dst, b"named_group is here");
}

#[test]
fn test_expand_with_invalid_capture_group() {
    struct CaptureGroup {
        groups: Vec<Option<Vec<u8>>>,
    }

    impl CaptureGroup {
        fn new() -> Self {
            Self { groups: vec![Some(b"match".to_vec())] }
        }

        fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>) {
            expand_bytes(self, replacement, dst)
        }
    }

    let capture_group = CaptureGroup::new();
    let mut dst = Vec::new();
    capture_group.expand(b"$1 and $name", &mut dst);
    assert_eq!(dst, b" and ");
}

#[test]
fn test_expand_with_literal_dollar_sign() {
    struct CaptureGroup {
        groups: Vec<Option<Vec<u8>>>,
    }

    impl CaptureGroup {
        fn new() -> Self {
            Self { groups: vec![Some(b"match".to_vec())] }
        }

        fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>) {
            expand_bytes(self, replacement, dst)
        }
    }

    let capture_group = CaptureGroup::new();
    let mut dst = Vec::new();
    capture_group.expand(b"$$$0", &mut dst);
    assert_eq!(dst, b"$match");
}

