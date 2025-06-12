// Answer 0

#[test]
fn test_writer_new_with_string() {
    let buffer = String::from("Hello, World!");
    let writer = new(buffer);
    assert_eq!(writer.buf, "Hello, World!");
}

#[test]
fn test_writer_new_with_vec_u8() {
    let buffer = vec![1, 2, 3, 4, 5];
    let writer = new(buffer);
    assert_eq!(writer.buf, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_writer_new_with_empty_string() {
    let buffer = String::new();
    let writer = new(buffer);
    assert_eq!(writer.buf, "");
}

#[test]
fn test_writer_new_with_empty_vec() {
    let buffer: Vec<u8> = Vec::new();
    let writer = new(buffer);
    assert!(writer.buf.is_empty());
}

#[test]
fn test_writer_new_with_integer() {
    let buffer = 42;
    let writer = new(buffer);
    assert_eq!(writer.buf, 42);
}

