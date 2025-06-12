// Answer 0

#[test]
fn test_reader_new_with_integer_buffer() {
    struct IntegerBuffer {
        data: Vec<i32>,
    }

    let buffer = IntegerBuffer { data: vec![1, 2, 3] };
    let reader = new(buffer);
    assert_eq!(reader.buf.data, vec![1, 2, 3]);
}

#[test]
fn test_reader_new_with_string_buffer() {
    struct StringBuffer {
        data: String,
    }

    let buffer = StringBuffer { data: String::from("hello") };
    let reader = new(buffer);
    assert_eq!(reader.buf.data, String::from("hello"));
}

#[test]
fn test_reader_new_with_empty_buffer() {
    struct EmptyBuffer {
        data: Vec<u8>,
    }

    let buffer = EmptyBuffer { data: vec![] };
    let reader = new(buffer);
    assert_eq!(reader.buf.data, vec![]);
}

