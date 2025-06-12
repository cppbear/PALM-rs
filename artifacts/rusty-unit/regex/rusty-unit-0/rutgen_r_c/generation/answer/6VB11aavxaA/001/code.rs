// Answer 0

#[test]
fn test_captures_debug_fmt() {
    use std::collections::HashMap;
    use std::sync::Arc;
    
    // Initialize a sample text and locations
    let sample_text = "Sample text for testing.";
    let sample_locations = Locations(vec![]);
    
    // Create a named groups HashMap
    let mut named_groups_map = HashMap::new();
    named_groups_map.insert(String::from("group1"), 0);
    
    // Wrap HashMap in an Arc
    let named_groups = Arc::new(named_groups_map);
    
    // Create a Captures instance
    let captures = Captures {
        text: sample_text,
        locs: sample_locations,
        named_groups: named_groups.clone(),
    };
    
    // Create a formatter for testing
    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    
    // Call the fmt function
    let result = captures.fmt(&mut formatter);

    // Assert that Result is Ok and output is formatted correctly
    assert!(result.is_ok());
    assert!(output.contains("Captures"));
}

#[test]
fn test_empty_captures_debug_fmt() {
    use std::collections::HashMap;
    use std::sync::Arc;

    // Initialize an empty text and locations
    let sample_text = "";
    let sample_locations = Locations(vec![]);

    // Create an empty named groups HashMap
    let named_groups_map = HashMap::new();
    let named_groups = Arc::new(named_groups_map);

    // Create a Captures instance
    let captures = Captures {
        text: sample_text,
        locs: sample_locations,
        named_groups: named_groups.clone(),
    };

    // Create a formatter for testing
    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);

    // Call the fmt function
    let result = captures.fmt(&mut formatter);

    // Assert that Result is Ok and output is formatted correctly
    assert!(result.is_ok());
    assert!(output.contains("Captures"));
}

