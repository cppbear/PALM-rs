// Answer 0

#[test]
fn test_replace_append_with_normal_input() {
    let input_data = b"test data";
    let captures = Captures {
        text: b"original text",
        locs: Locations::default(), // Assuming Locations can be initialized like this
        named_groups: Arc::new(HashMap::new()),
    };
    let mut output = Vec::new();
    let mut replacer = NoExpand(input_data);

    replacer.replace_append(&captures, &mut output);
    assert_eq!(output, b"test data");
}

#[test]
fn test_replace_append_with_empty_input() {
    let input_data = b"";
    let captures = Captures {
        text: b"original text",
        locs: Locations::default(), // Assuming Locations can be initialized like this
        named_groups: Arc::new(HashMap::new()),
    };
    let mut output = Vec::new();
    let mut replacer = NoExpand(input_data);

    replacer.replace_append(&captures, &mut output);
    assert_eq!(output, b"");
}

#[test]
fn test_replace_append_with_large_input() {
    let input_data = b"large input string that is significantly longer than usual";
    let captures = Captures {
        text: b"original text",
        locs: Locations::default(), // Assuming Locations can be initialized like this
        named_groups: Arc::new(HashMap::new()),
    };
    let mut output = Vec::new();
    let mut replacer = NoExpand(input_data);

    replacer.replace_append(&captures, &mut output);
    assert_eq!(output, b"large input string that is significantly longer than usual");
}

