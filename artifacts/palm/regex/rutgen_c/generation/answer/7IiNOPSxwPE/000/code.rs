// Answer 0

#[test]
fn test_expand_str_with_number_reference() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let text = "hello world";
    let locs = Locations::new(); // Initialize Locations appropriately for your context
    let named_groups = Arc::new(HashMap::new());

    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let mut dst = String::new();
    expand_str(&captures, "$0 test", &mut dst);
    assert_eq!(dst, "hello world test");
}

#[test]
fn test_expand_str_with_named_reference() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let text = "hello world";
    let locs = Locations::new(); // Initialize Locations appropriately for your context
    let mut named_groups = HashMap::new();
    named_groups.insert("name".to_string(), 0);
    let named_groups = Arc::new(named_groups);

    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let mut dst = String::new();
    expand_str(&captures, "${name} test", &mut dst);
    assert_eq!(dst, "hello world test");
}

#[test]
fn test_expand_str_with_missing_reference() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let text = "hello world";
    let locs = Locations::new(); // Initialize Locations appropriately for your context
    let named_groups = Arc::new(HashMap::new());

    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let mut dst = String::new();
    expand_str(&captures, "$1 test", &mut dst);
    assert_eq!(dst, "$1 test");

    let mut dst_named = String::new();
    expand_str(&captures, "${notfound} test", &mut dst_named);
    assert_eq!(dst_named, "$notfound test");
}

#[test]
fn test_expand_str_with_double_dollar() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let text = "hello world";
    let locs = Locations::new(); // Initialize Locations appropriately for your context
    let named_groups = Arc::new(HashMap::new());

    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let mut dst = String::new();
    expand_str(&captures, "$$ test", &mut dst);
    assert_eq!(dst, "$ test");
}

