// Answer 0

#[test]
fn test_reader_new_with_vec() {
    let buf: Vec<u8> = vec![1, 2, 3];
    let reader = new(buf);
    assert_eq!(reader.buf, vec![1, 2, 3]);
}

#[test]
fn test_reader_new_with_array() {
    let buf: [u8; 3] = [1, 2, 3];
    let reader = new(buf);
    assert_eq!(reader.buf, [1, 2, 3]);
}

#[test]
fn test_reader_new_with_string() {
    let buf: String = String::from("test");
    let reader = new(buf);
    assert_eq!(reader.buf, "test");
}

#[test]
fn test_reader_new_with_empty_vec() {
    let buf: Vec<u8> = vec![];
    let reader = new(buf);
    assert_eq!(reader.buf, vec![]);
}

#[test]
fn test_reader_new_with_large_data() {
    let buf: Vec<u8> = (0..1_000).collect();
    let reader = new(buf.clone());
    assert_eq!(reader.buf, buf);
}

