// Answer 0

#[test]
fn test_expand_str_with_named_capture() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let text = "Hello, World!";
    let named_groups = Arc::new(HashMap::new());
    let mut locs = Locations::new();  // This requires a concrete implementation of Locations
    locs.add(0, 5); // Assuming 0-5 is a valid capture range for the first capture

    let captures = re_unicode::Captures {
        text,
        locs,
        named_groups,
    };

    let replacement = "Greeting: ${greeting}";  
    let mut dst = String::new();
    expand_str(&captures, replacement, &mut dst);
    assert_eq!(dst, "Greeting: "); // Assuming that ${greeting} does not match any named capture
}

#[test]
fn test_expand_str_with_empty_replacement() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let text = "Hello";
    let named_groups = Arc::new(HashMap::new());
    let mut locs = Locations::new();  // This requires a concrete implementation of Locations
    locs.add(0, 5); // Assuming 0-5 is a valid capture range for the first capture

    let captures = re_unicode::Captures {
        text,
        locs,
        named_groups,
    };

    let replacement = "No capture here";  
    let mut dst = String::new();
    expand_str(&captures, replacement, &mut dst);
    assert_eq!(dst, "No capture here"); // Full replacement string directly copied
}

#[test]
fn test_expand_str_with_double_dollar() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let text = "Hello, World!";
    let named_groups = Arc::new(HashMap::new());
    let mut locs = Locations::new();  // This requires a concrete implementation of Locations
    locs.add(0, 5); // Assuming 0-5 is a valid capture range for the first capture

    let captures = re_unicode::Captures {
        text,
        locs,
        named_groups,
    };

    let replacement = "Price: $$100";  
    let mut dst = String::new();
    expand_str(&captures, replacement, &mut dst);
    assert_eq!(dst, "Price: $100"); // Double dollar should translate to a single dollar
}

