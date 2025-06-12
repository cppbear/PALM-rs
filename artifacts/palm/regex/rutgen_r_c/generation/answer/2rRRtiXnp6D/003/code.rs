// Answer 0

#[test]
fn test_expand_bytes_single_capture_number() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct DummyCaptures {
        matches: Vec<Option<&'static [u8]>>,
    }

    impl re_bytes::Captures for DummyCaptures {
        fn get(&self, i: usize) -> Option<re_bytes::Match<'_>> {
            self.matches.get(i).map(|&m| re_bytes::Match {
                text: m.unwrap_or(b""),
                start: 0,
                end: m.map_or(0, |v| v.len()),
            })
        }
    }

    let caps = DummyCaptures {
        matches: vec![Some(b"Hello"), None, Some(b"World")],
    };

    let mut dst = Vec::new();
    expand_bytes(&caps, b"$0 and $2", &mut dst);
    let result = String::from_utf8(dst).expect("valid UTF-8");
    assert_eq!(result, "Hello and World");
}

#[test]
fn test_expand_bytes_double_dollar_sign() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct DummyCaptures {
        matches: Vec<Option<&'static [u8]>>,
    }

    impl re_bytes::Captures for DummyCaptures {
        fn get(&self, i: usize) -> Option<re_bytes::Match<'_>> {
            self.matches.get(i).map(|&m| re_bytes::Match {
                text: m.unwrap_or(b""),
                start: 0,
                end: m.map_or(0, |v| v.len()),
            })
        }
    }

    let caps = DummyCaptures {
        matches: vec![Some(b"Test")],
    };

    let mut dst = Vec::new();
    expand_bytes(&caps, b"$Test and $$", &mut dst);
    let result = String::from_utf8(dst).expect("valid UTF-8");
    assert_eq!(result, "Test and $");
}

#[test]
fn test_expand_bytes_empty_replacement() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct DummyCaptures {
        matches: Vec<Option<&'static [u8]>>,
    }

    impl re_bytes::Captures for DummyCaptures {
        fn get(&self, i: usize) -> Option<re_bytes::Match<'_>> {
            self.matches.get(i).map(|&m| re_bytes::Match {
                text: m.unwrap_or(b""),
                start: 0,
                end: m.map_or(0, |v| v.len()),
            })
        }
    }

    let caps = DummyCaptures {
        matches: vec![Some(b"Capture")],
    };

    let mut dst = Vec::new();
    expand_bytes(&caps, b"", &mut dst);
    let result = String::from_utf8(dst).expect("valid UTF-8");
    assert_eq!(result, "");
}

#[test]
fn test_expand_bytes_invalid_capture() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct DummyCaptures {
        matches: Vec<Option<&'static [u8]>>,
    }

    impl re_bytes::Captures for DummyCaptures {
        fn get(&self, i: usize) -> Option<re_bytes::Match<'_>> {
            self.matches.get(i).map(|&m| re_bytes::Match {
                text: m.unwrap_or(b""),
                start: 0,
                end: m.map_or(0, |v| v.len()),
            })
        }
    }

    let caps = DummyCaptures {
        matches: vec![None],
    };

    let mut dst = Vec::new();
    expand_bytes(&caps, b"$1", &mut dst);
    let result = String::from_utf8(dst).expect("valid UTF-8");
    assert_eq!(result, "");
}

#[test]
#[should_panic]
fn test_expand_bytes_invalid_access_before_match() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct DummyCaptures {
        matches: Vec<Option<&'static [u8]>>,
    }

    impl re_bytes::Captures for DummyCaptures {
        fn get(&self, i: usize) -> Option<re_bytes::Match<'_>> {
            self.matches.get(i).map(|&m| re_bytes::Match {
                text: m.unwrap_or(b""),
                start: 0,
                end: m.map_or(0, |v| v.len()),
            })
        }
    }

    let caps = DummyCaptures {
        matches: vec![Some(b"Valid")],
    };

    let mut dst = Vec::new();
    expand_bytes(&caps, b"$2", &mut dst); // Invalid access
}

