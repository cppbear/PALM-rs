// Answer 0

#[test]
fn test_expand_str_basic() {
    use std::collections::HashMap;
    use std::sync::Arc;

    struct MockCaptures<'a> {
        text: &'a str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let mut named_groups = HashMap::new();
    named_groups.insert("name".to_string(), 0);
    let captures = MockCaptures {
        text: "hello world",
        locs: Locations::new(vec![(0, 5), (6, 11)]), // Example locations
        named_groups: Arc::new(named_groups),
    };

    let mut dst = String::new();
    let replacement = "$name and $0";
    expand_str(&captures, replacement, &mut dst);

    assert_eq!(dst, "hello world and hello"); // checks that the expansion worked correctly
}

#[test]
#[should_panic]
fn test_expand_str_invalid_replacement_format() {
    use std::collections::HashMap;
    use std::sync::Arc;

    struct MockCaptures<'a> {
        text: &'a str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let mut named_groups = HashMap::new();
    named_groups.insert("name".to_string(), 0);
    let captures = MockCaptures {
        text: "hello world",
        locs: Locations::new(vec![(0, 5), (6, 11)]),
        named_groups: Arc::new(named_groups),
    };

    let mut dst = String::new();
    let replacement = "$name and $invalid"; // Invalid reference should panic
    expand_str(&captures, replacement, &mut dst);
}

#[test]
fn test_expand_str_empty_replacement() {
    use std::collections::HashMap;
    use std::sync::Arc;

    struct MockCaptures<'a> {
        text: &'a str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let mut named_groups = HashMap::new();
    named_groups.insert("name".to_string(), 0);
    let captures = MockCaptures {
        text: "hello world",
        locs: Locations::new(vec![(0, 5), (6, 11)]),
        named_groups: Arc::new(named_groups),
    };

    let mut dst = String::new();
    let replacement = ""; // empty replacement check
    expand_str(&captures, replacement, &mut dst);

    assert_eq!(dst, ""); // should remain empty
}

