// Answer 0

#[test]
fn test_from_str_valid_signed_number() {
    let input = "42";
    let result: Result<i32, serde_json::Error> = serde_json::from_str(input);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_from_str_negative_signed_number() {
    let input = "-99";
    let result: Result<i32, serde_json::Error> = serde_json::from_str(input);
    assert_eq!(result, Ok(-99));
}

#[test]
fn test_from_str_zero_signed_number() {
    let input = "0";
    let result: Result<i32, serde_json::Error> = serde_json::from_str(input);
    assert_eq!(result, Ok(0));
}

#[test]
#[should_panic]
fn test_from_str_invalid_signed_number() {
    let input = "invalid";
    let _: Result<i32, serde_json::Error> = serde_json::from_str(input).unwrap();
}

#[test]
#[should_panic]
fn test_from_str_empty_string() {
    let input = "";
    let _: Result<i32, serde_json::Error> = serde_json::from_str(input).unwrap();
}

#[test]
fn test_from_str_large_signed_number() {
    let input = "2147483647"; // Max i32
    let result: Result<i32, serde_json::Error> = serde_json::from_str(input);
    assert_eq!(result, Ok(2147483647));
}

#[test]
fn test_from_str_small_signed_number() {
    let input = "-2147483648"; // Min i32
    let result: Result<i32, serde_json::Error> = serde_json::from_str(input);
    assert_eq!(result, Ok(-2147483648));
}

