// Answer 0

#[test]
fn test_fmt_pretty_json() {
    use serde_json::json;
    use std::fmt;

    // Initialize a JSON object with multiple fields
    let json_obj = json!({ "city": "London", "street": "10 Downing Street" });

    // Prepare a buffer to hold the formatted output
    let mut output = String::new();
    
    // Use the formatter to format the JSON object in pretty format
    write!(&mut output, "{:#}", json_obj).unwrap();

    // Verify the expected pretty format output
    let expected = "{\n  \"city\": \"London\",\n  \"street\": \"10 Downing Street\"\n}";
    assert_eq!(output, expected);
}

#[test]
fn test_fmt_pretty_empty_json() {
    use serde_json::json;
    use std::fmt;

    // Initialize an empty JSON object
    let json_obj = json!({ });

    // Prepare a buffer to hold the formatted output
    let mut output = String::new();
    
    // Use the formatter to format the empty JSON object in pretty format
    write!(&mut output, "{:#}", json_obj).unwrap();

    // Verify the expected pretty format output for an empty object
    let expected = "{}";
    assert_eq!(output, expected);
}

#[test]
fn test_fmt_pretty_nested_json() {
    use serde_json::json;
    use std::fmt;

    // Initialize a nested JSON object
    let json_obj = json!({ 
        "city": "London", 
        "address": { 
            "street": "10 Downing Street", 
            "postalCode": "SW1A 2AA" 
        } 
    });

    // Prepare a buffer to hold the formatted output
    let mut output = String::new();
    
    // Use the formatter to format the nested JSON object in pretty format
    write!(&mut output, "{:#}", json_obj).unwrap();

    // Verify the expected pretty format output
    let expected = "{\n  \"address\": {\n    \"postalCode\": \"SW1A 2AA\",\n    \"street\": \"10 Downing Street\"\n  },\n  \"city\": \"London\"\n}";
    assert_eq!(output, expected);
}

