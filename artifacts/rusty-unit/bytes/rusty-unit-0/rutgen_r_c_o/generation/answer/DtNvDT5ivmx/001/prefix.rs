// Answer 0

#[test]
fn test_split_to_valid_range_1() {
    let mut b1 = Bytes::from("hello world");
    b1.split_to(1);
}

#[test]
fn test_split_to_valid_range_100() {
    let mut b1 = Bytes::from("a".repeat(100));
    b1.split_to(100);
}

#[test]
fn test_split_to_valid_range_101() {
    let mut b1 = Bytes::from("a".repeat(101));
    b1.split_to(101);
}

#[test]
fn test_split_to_valid_range_200() {
    let mut b1 = Bytes::from("a".repeat(200));
    b1.split_to(200);
}

#[test]
fn test_split_to_valid_range_300() {
    let mut b1 = Bytes::from("a".repeat(300));
    b1.split_to(300);
}

#[test]
fn test_split_to_valid_range_400() {
    let mut b1 = Bytes::from("a".repeat(400));
    b1.split_to(400);
}

#[test]
fn test_split_to_valid_range_500() {
    let mut b1 = Bytes::from("a".repeat(500));
    b1.split_to(500);
}

#[test]
fn test_split_to_valid_range_600() {
    let mut b1 = Bytes::from("a".repeat(600));
    b1.split_to(600);
}

#[test]
fn test_split_to_invalid_range_overflow() {
    let mut b1 = Bytes::from("hello world");
    let result = panic::catch_unwind(|| {
        b1.split_to(15);
    });
    assert!(result.is_err());
}

