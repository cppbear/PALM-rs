// Answer 0

#[test]
fn test_try_append_success() {
    struct CustomValue;
    let mut header_map: HeaderMap<CustomValue> = HeaderMap {
        mask: Size::default(),
        indices: Box::new([]),
        entries: Vec::new(),
        extra_values: Vec::new(),
        danger: Danger::default(),
    };

    let header_name: &'static str = "Test-Header";
    let custom_value = CustomValue;

    let result = header_name.try_append(&mut header_map, custom_value);

    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_try_append_max_size_reached() {
    struct CustomValue;
    let mut header_map: HeaderMap<CustomValue> = HeaderMap {
        mask: Size::default(),
        indices: Box::new([]),
        entries: Vec::new(),
        extra_values: Vec::new(),
        danger: Danger::default(),
    };

    let header_name: &'static str = "Test-Header";
    let custom_value = CustomValue;

    // Simulate reaching the maximum size before appending
    header_map.mask = Size::maximum(); // Assuming Size has a method to set maximum size

    let result = header_name.try_append(&mut header_map, custom_value);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err()._priv, ()); // Adjust as needed to check the error specifics
}

