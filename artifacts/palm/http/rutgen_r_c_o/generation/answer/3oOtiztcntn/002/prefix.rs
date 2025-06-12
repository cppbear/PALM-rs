// Answer 0

#[test]
fn test_append_value_empty_links() {
    let entry_idx = 0;
    let mut entry = Bucket {
        hash: HashValue::default(),
        key: HeaderName::from_static("Test-Header"),
        value: "InitialValue",
        links: None,
    };
    let mut extra: Vec<ExtraValue<&str>> = Vec::new();
    let value = "NewValue";

    append_value(entry_idx, &mut entry, &mut extra, value);
}

#[test]
fn test_append_value_with_one_link() {
    let entry_idx = 1;
    let mut entry = Bucket {
        hash: HashValue::default(),
        key: HeaderName::from_static("Another-Header"),
        value: "InitialValue",
        links: None,
    };
    let mut extra: Vec<ExtraValue<&str>> = Vec::new();
    let value = "AdditionalValue";

    append_value(entry_idx, &mut entry, &mut extra, value);
}

#[test]
fn test_append_value_multiple_entries() {
    let entry_idx = 2;
    let mut entry = Bucket {
        hash: HashValue::default(),
        key: HeaderName::from_static("Third-Header"),
        value: "InitialValue",
        links: None,
    };
    let mut extra: Vec<ExtraValue<&str>> = Vec::new();
    
    append_value(entry_idx, &mut entry, &mut extra, "FirstValue");
    append_value(entry_idx, &mut entry, &mut extra, "SecondValue");
}

#[test]
fn test_append_value_large_index() {
    let entry_idx = 65535; // Max value for Size
    let mut entry = Bucket {
        hash: HashValue::default(),
        key: HeaderName::from_static("Max-Index-Header"),
        value: "InitialValue",
        links: None,
    };
    let mut extra: Vec<ExtraValue<&str>> = Vec::new();
    let value = "ValueAtMaxIndex";

    append_value(entry_idx, &mut entry, &mut extra, value);
}

#[test]
fn test_append_value_various_types() {
    let entry_idx = 3;
    let mut entry = Bucket {
        hash: HashValue::default(),
        key: HeaderName::from_static("Varied-Type-Header"),
        value: 42, // Using an integer as value
        links: None,
    };
    let mut extra: Vec<ExtraValue<i32>> = Vec::new();
    append_value(entry_idx, &mut entry, &mut extra, 100);
}

