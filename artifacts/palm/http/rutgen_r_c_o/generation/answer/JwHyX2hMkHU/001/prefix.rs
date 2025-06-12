// Answer 0

#[test]
fn test_map_with_string_body() {
    let response = Response::new("some string");
    let mapped_response: Response<Vec<u8>> = response.map(|b| b.as_bytes().to_vec());
}

#[test]
fn test_map_with_integer_body() {
    let response = Response::new(42);
    let mapped_response: Response<String> = response.map(|b| b.to_string());
}

#[test]
fn test_map_with_empty_string_body() {
    let response = Response::new("");
    let mapped_response: Response<Vec<u8>> = response.map(|b| b.as_bytes().to_vec());
}

#[test]
fn test_map_with_large_body() {
    let response = Response::new("a".repeat(1_000_000));
    let mapped_response: Response<Vec<u8>> = response.map(|b| b.as_bytes().to_vec());
}

#[test]
fn test_map_with_none_body() {
    let response = Response::new(Option::<&str>::None);
    let mapped_response: Response<Option<i32>> = response.map(|b| b.map(|_| 0));
}

