// Answer 0

#[test]
fn test_from_iter_with_vector() {
    let input = vec![1, 2, 3, 4, 5];
    let result: Vec<u8> = from_iter(input.clone());
    assert_eq!(result, input.into_iter().collect::<Vec<u8>>());
}

#[test]
fn test_from_iter_with_array() {
    let input = [10, 20, 30];
    let result: Vec<u8> = from_iter(input.iter().cloned());
    assert_eq!(result, input.to_vec());
}

#[test]
fn test_from_iter_with_slice() {
    let input: &[u8] = &[100, 200, 255];
    let result: Vec<u8> = from_iter(input);
    assert_eq!(result, input.to_vec());
}

#[test]
fn test_from_iter_empty() {
    let input: Vec<u8> = Vec::new();
    let result: Vec<u8> = from_iter(input);
    assert!(result.is_empty());
}

