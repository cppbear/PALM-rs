// Answer 0

#[test]
fn test_replace_append_empty_captures() {
    struct DummyReplacer;

    let mut replacer = |caps: &Captures| -> String {
        format!("Replaced: {}", caps.text)
    };

    let text = "";
    let locs = Locations::new();
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let mut dst = String::new();
    replacer.replace_append(&captures, &mut dst);

    assert_eq!(dst, "Replaced: ");
}

#[test]
fn test_replace_append_single_capture() {
    struct DummyReplacer;

    let mut replacer = |caps: &Captures| -> String {
        format!("Replaced: {}", caps.text)
    };

    let text = "test";
    let locs = Locations::new();
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let mut dst = String::new();
    replacer.replace_append(&captures, &mut dst);

    assert_eq!(dst, "Replaced: test");
}

#[test]
fn test_replace_append_multi_capture() {
    struct DummyReplacer;

    let mut replacer = |caps: &Captures| -> String {
        format!("Replaced: {} and more", caps.text)
    };

    let text = "sample text";
    let locs = Locations::new();
    let named_groups = Arc::new(HashMap::new());
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let mut dst = String::new();
    replacer.replace_append(&captures, &mut dst);

    assert_eq!(dst, "Replaced: sample text and more");
}

