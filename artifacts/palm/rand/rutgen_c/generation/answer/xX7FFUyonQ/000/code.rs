// Answer 0

#[test]
fn test_array64_debug_format() {
    let array = Array64([0u32; 64]);
    let result = format!("{:?}", array);
    assert_eq!(result, "Array64 {{}}");
}

