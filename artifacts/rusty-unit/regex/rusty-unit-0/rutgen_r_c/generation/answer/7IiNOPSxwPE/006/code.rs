// Answer 0

fn test_expand_str_valid_replacement() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct MockCaptures<'t> {
        text: &'t str,
        locs: Locations, // Assume Locations struct is defined in scope
        named_groups: Arc<HashMap<String, usize>>,
    }

    let text = "Hello, world!";
    let named_groups = Arc::new(HashMap::new());
    let captures = MockCaptures {
        text,
        locs: Locations::new(), // Assume Locations::new() initializes a valid Locations instance
        named_groups,
    };

    let mut dst = String::new();
    let replacement = "Greetings, $1!";

    expand_str(&captures, replacement, &mut dst);
    assert_eq!(dst, "Greetings, !");
}

fn test_expand_str_with_empty_replacement() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct MockCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let text = "Foo";
    let named_groups = Arc::new(HashMap::new());
    let captures = MockCaptures {
        text,
        locs: Locations::new(),
        named_groups,
    };

    let mut dst = String::new();
    let replacement = "$";

    expand_str(&captures, replacement, &mut dst);
    assert_eq!(dst, "$");
}

fn test_expand_str_with_invalid_capture() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct MockCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let text = "Test";
    let named_groups = Arc::new(HashMap::new());
    let captures = MockCaptures {
        text,
        locs: Locations::new(),
        named_groups,
    };

    let mut dst = String::new();
    let replacement = "Invalid $42 string";

    expand_str(&captures, replacement, &mut dst);
    assert_eq!(dst, "Invalid  string");
}

fn test_expand_str_with_empty_final_replacement() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct MockCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let text = "Word";
    let named_groups = Arc::new(HashMap::new());
    let captures = MockCaptures {
        text,
        locs: Locations::new(),
        named_groups,
    };

    let mut dst = String::new();
    let replacement = "Captures: $1}";

    expand_str(&captures, replacement, &mut dst);
    assert_eq!(dst, "Captures: ");
}

fn test_expand_str_with_consecutive_dollar_signs() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct MockCaptures<'t> {
        text: &'t str,
        locs: Locations,
        named_groups: Arc<HashMap<String, usize>>,
    }

    let text = "Bar";
    let named_groups = Arc::new(HashMap::new());
    let captures = MockCaptures {
        text,
        locs: Locations::new(),
        named_groups,
    };

    let mut dst = String::new();
    let replacement = "Hello $$ World";

    expand_str(&captures, replacement, &mut dst);
    assert_eq!(dst, "Hello $ World");
}

