// Answer 0

#[derive(Debug)]
struct Take<T> {
    inner: T,
    limit: usize,
}

#[test]
fn test_take_new() {
    let inner_value = vec![1, 2, 3, 4, 5];
    let limit = 3;

    let take_instance = new(inner_value.clone(), limit);

    assert_eq!(take_instance.inner, inner_value);
    assert_eq!(take_instance.limit, limit);
}

#[test]
fn test_take_new_limit_zero() {
    let inner_value = vec![1, 2, 3, 4, 5];
    let limit = 0;

    let take_instance = new(inner_value.clone(), limit);

    assert_eq!(take_instance.inner, inner_value);
    assert_eq!(take_instance.limit, limit);
}

#[test]
fn test_take_new_limit_exceeds_length() {
    let inner_value = vec![1, 2, 3];
    let limit = 5;

    let take_instance = new(inner_value.clone(), limit);

    assert_eq!(take_instance.inner, inner_value);
    assert_eq!(take_instance.limit, limit);
}

