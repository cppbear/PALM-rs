// Answer 0

#[test]
fn test_split_off_valid_index() {
    let mut b1 = BytesMut::from("hello world");
    b1.split_off(6);
}

#[test]
fn test_split_off_edge_case_start() {
    let mut b1 = BytesMut::from("hello world");
    b1.split_off(1);
}

#[test]
fn test_split_off_edge_case_end() {
    let mut b1 = BytesMut::from("hello world");
    b1.split_off(11);
}

#[test]
fn test_split_off_middle() {
    let mut b1 = BytesMut::from("hello world");
    b1.split_off(5);
}

#[should_panic]
fn test_split_off_panic_index_zero() {
    let mut b1 = BytesMut::from("hello world");
    b1.split_off(0);
}

#[should_panic]
fn test_split_off_panic_out_of_bounds() {
    let mut b1 = BytesMut::from("hello world");
    b1.split_off(12);
}

#[test]
fn test_split_off_empty() {
    let mut b1 = BytesMut::from("");
    b1.split_off(1);
}

#[test]
fn test_split_off_large_input() {
    let mut b1 = BytesMut::from("A".repeat(1024));
    b1.split_off(512);
}

#[should_panic]
fn test_split_off_large_input_zero_index() {
    let mut b1 = BytesMut::from("A".repeat(1024));
    b1.split_off(0);
}

