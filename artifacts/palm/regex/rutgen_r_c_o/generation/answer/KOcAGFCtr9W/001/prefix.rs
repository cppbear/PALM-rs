// Answer 0

#[test]
fn test_replace_append_empty_caps() {
    struct TestReplacer;

    let caps = Captures {
        text: b"",
        locs: Locations::default(),
        named_groups: Arc::new(HashMap::new()),
    };
    
    let mut dst = Vec::new();
    let mut replacer = TestReplacer;

    replacer.replace_append(&caps, &mut dst);
}

#[test]
fn test_replace_append_single_byte() {
    struct TestReplacer;

    let caps = Captures {
        text: b"a",
        locs: Locations::default(),
        named_groups: Arc::new(HashMap::new()),
    };
    
    let mut dst = Vec::new();
    let mut replacer = TestReplacer;

    replacer.replace_append(&caps, &mut dst);
}

#[test]
fn test_replace_append_multiple_bytes() {
    struct TestReplacer;

    let caps = Captures {
        text: b"abcde",
        locs: Locations::default(),
        named_groups: Arc::new(HashMap::new()),
    };
    
    let mut dst = Vec::new();
    let mut replacer = TestReplacer;

    replacer.replace_append(&caps, &mut dst);
}

#[test]
fn test_replace_append_large_caps() {
    struct TestReplacer;

    let text = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let caps = Captures {
        text: &text[..1000],
        locs: Locations::default(),
        named_groups: Arc::new(HashMap::new()),
    };
    
    let mut dst = Vec::new();
    let mut replacer = TestReplacer;

    replacer.replace_append(&caps, &mut dst);
}

#[test]
fn test_replace_append_existing_dst() {
    struct TestReplacer;

    let caps = Captures {
        text: b"xyz",
        locs: Locations::default(),
        named_groups: Arc::new(HashMap::new()),
    };
    
    let mut dst = vec![b'1', b'2', b'3'];
    let mut replacer = TestReplacer;

    replacer.replace_append(&caps, &mut dst);
}

#[test]
fn test_replace_append_panic_on_empty_dst() {
    struct TestReplacer;

    let caps = Captures {
        text: b"",
        locs: Locations::default(),
        named_groups: Arc::new(HashMap::new()),
    };
    
    let mut dst = Vec::with_capacity(0);
    let mut replacer = TestReplacer;

    replacer.replace_append(&caps, &mut dst);
}

