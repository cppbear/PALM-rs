// Answer 0

#[test]
fn test_as_bytes_valid_range() {
    let haystack: &[u8] = b"hello";
    let m = Match::new(haystack, 0, 5);
    m.as_bytes();
}

#[test]
fn test_as_bytes_single_byte() {
    let haystack: &[u8] = b"hello";
    let m = Match::new(haystack, 1, 2);
    m.as_bytes();
}

#[test]
fn test_as_bytes_empty_range() {
    let haystack: &[u8] = b"hello";
    let m = Match::new(haystack, 0, 0);
    m.as_bytes();
}

#[test]
fn test_as_bytes_full_range() {
    let haystack: &[u8] = b"hello";
    let m = Match::new(haystack, 0, 5);
    m.as_bytes();
}

#[test]
#[should_panic]
fn test_as_bytes_out_of_bounds_start() {
    let haystack: &[u8] = b"hello";
    let m = Match::new(haystack, 6, 5);
    m.as_bytes();
}

#[test]
#[should_panic]
fn test_as_bytes_out_of_bounds_end() {
    let haystack: &[u8] = b"hello";
    let m = Match::new(haystack, 0, 6);
    m.as_bytes();
}

#[test]
#[should_panic]
fn test_as_bytes_end_before_start() {
    let haystack: &[u8] = b"hello";
    let m = Match::new(haystack, 3, 2);
    m.as_bytes();
}

