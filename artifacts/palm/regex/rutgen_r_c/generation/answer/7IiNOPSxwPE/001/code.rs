// Answer 0

#[test]
fn test_expand_str_empty_replacement() {
    use std::sync::Arc;
    use std::collections::HashMap;

    // Create dummy Captures with empty named groups and locations
    struct Locations;

    impl Locations {
        fn pos(&self, _: usize) -> Option<(usize, usize)> {
            None // No captures
        }
    }

    let empty_text = "";
    let named_groups = Arc::new(HashMap::<String, usize>::new());
    let caps = Captures {
        text: empty_text,
        locs: Locations,
        named_groups,
    };

    let mut dst = String::new();
    let replacement = "";

    expand_str(&caps, replacement, &mut dst);

    // Expected behavior: Since the replacement string is empty,
    // the destination should remain empty as well.
    assert_eq!(dst, "");
}

#[test]
fn test_expand_str_only_dollar() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct Locations;

    impl Locations {
        fn pos(&self, _: usize) -> Option<(usize, usize)> {
            None // No captures
        }
    }

    let empty_text = "";
    let named_groups = Arc::new(HashMap::<String, usize>::new());
    let caps = Captures {
        text: empty_text,
        locs: Locations,
        named_groups,
    };

    let mut dst = String::new();
    let replacement = "$";

    expand_str(&caps, replacement, &mut dst);

    // Expected behavior: the dollar sign alone should be added to the destination.
    assert_eq!(dst, "$");
}

#[test]
fn test_expand_str_double_dollar() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct Locations;

    impl Locations {
        fn pos(&self, _: usize) -> Option<(usize, usize)> {
            None // No captures
        }
    }

    let empty_text = "";
    let named_groups = Arc::new(HashMap::<String, usize>::new());
    let caps = Captures {
        text: empty_text,
        locs: Locations,
        named_groups,
    };

    let mut dst = String::new();
    let replacement = "$$";

    expand_str(&caps, replacement, &mut dst);

    // Expected behavior: two dollar signs should result in one dollar sign.
    assert_eq!(dst, "$");
}

#[test]
fn test_expand_str_no_capture() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct Locations;

    impl Locations {
        fn pos(&self, _: usize) -> Option<(usize, usize)> {
            None // No captures
        }
    }

    let empty_text = "";
    let named_groups = Arc::new(HashMap::<String, usize>::new());
    let caps = Captures {
        text: empty_text,
        locs: Locations,
        named_groups,
    };

    let mut dst = String::new();
    let replacement = "$1 and $name";

    expand_str(&caps, replacement, &mut dst);

    // Expected behavior: since there are no captures, we should see $1 and $name in destination.
    assert_eq!(dst, "$1 and $name");
}

