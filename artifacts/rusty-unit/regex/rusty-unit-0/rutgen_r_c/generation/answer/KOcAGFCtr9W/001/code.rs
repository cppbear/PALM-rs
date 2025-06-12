// Answer 0

#[test]
fn test_replace_append_empty_captures() {
    struct DummyReplacer;

    let mut replacer = DummyReplacer;
    let mut dst = Vec::new();
    
    let captures = Captures {
        text: b"",
        locs: Locations::default(),
        named_groups: Arc::new(HashMap::new()),
    };

    replacer.replace_append(&captures, &mut dst);
    assert_eq!(dst, b"");
}

#[test]
fn test_replace_append_single_match() {
    struct DummyReplacer;

    let mut replacer = |caps: &Captures| {
        if caps.len() > 0 {
            b"replacement".to_vec()
        } else {
            b"".to_vec()
        }
    };
    let mut dst = Vec::new();
    
    let named_groups = Arc::new(HashMap::from([("group1".to_string(), 0)]));
    let captures = Captures {
        text: b"matched text",
        locs: Locations::default(),
        named_groups: named_groups.clone(),
    };

    replacer.replace_append(&captures, &mut dst);
    assert_eq!(dst, b"replacement");
}

#[test]
fn test_replace_append_multiple_matches() {
    struct DummyReplacer;

    let mut replacer = |caps: &Captures| {
        let mut replacement = Vec::new();
        for _ in 0..caps.len() {
            replacement.extend_from_slice(b"replacement;");
        }
        replacement
    };
    let mut dst = Vec::new();
    
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures {
        text: b"matched text",
        locs: Locations::default(),
        named_groups,
    };

    replacer.replace_append(&captures, &mut dst);
    assert_eq!(dst, b"replacement;replacement;");
}

#[test]
fn test_replace_append_with_no_named_groups() {
    struct DummyReplacer;

    let mut replacer = |caps: &Captures| {
        b"no group".to_vec()
    };
    let mut dst = Vec::new();
    
    let captures = Captures {
        text: b"test",
        locs: Locations::default(),
        named_groups: Arc::new(HashMap::new()),
    };

    replacer.replace_append(&captures, &mut dst);
    assert_eq!(dst, b"no group");
}

