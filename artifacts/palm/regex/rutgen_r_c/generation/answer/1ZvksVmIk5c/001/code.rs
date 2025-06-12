// Answer 0

#[test]
fn test_captures_debug_fmt() {
    let text: &[u8] = b"sample text";
    let locs = Locations(vec![]); // Empty Locations for basic test
    let named_groups = Arc::new(HashMap::new()); // No named groups for basic test

    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let mut output = Vec::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    let result = captures.fmt(formatter);
    
    assert!(result.is_ok());
    let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");
    assert!(output_str.contains("Captures"));
}

#[test]
fn test_captures_debug_fmt_with_named_groups() {
    let text: &[u8] = b"sample text with named groups";
    let mut named_groups: HashMap<String, usize> = HashMap::new();
    named_groups.insert("group1".to_string(), 0);
    named_groups.insert("group2".to_string(), 1);
    let locs = Locations(vec![]); // Still keeping it simple

    let captures = Captures {
        text,
        locs,
        named_groups: Arc::new(named_groups),
    };

    let mut output = Vec::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    
    let result = captures.fmt(formatter);
    
    assert!(result.is_ok());
    let output_str = String::from_utf8(output).expect("Output is not valid UTF-8");
    assert!(output_str.contains("Captures"));
}

#[test]
#[should_panic]
fn test_captures_debug_fmt_panic_on_invalid_formatter() {
    let text: &[u8] = b"panic test";
    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());

    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let result = captures.fmt(&mut panic_formatter()); // Assuming `panic_formatter` causes panic
    drop(result); // To use the result and avoid warnings
}

fn panic_formatter() -> fmt::Formatter {
    // A formatter that is designed to panic, triggering a panic on use.
    struct PanicFormatter;

    impl fmt::Write for PanicFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            panic!("This formatter panics!");
        }
    }

    PanicFormatter
}

