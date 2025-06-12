// Answer 0

#[test]
fn test_writer_new_with_vec() {
    struct Writer<B> {
        buf: B,
    }
    
    let buffer: Vec<u8> = Vec::new();
    let writer = new(buffer);
    assert_eq!(writer.buf.len(), 0);
}

#[test]
fn test_writer_new_with_array() {
    struct Writer<B> {
        buf: B,
    }
    
    let buffer: [u8; 3] = [1, 2, 3];
    let writer = new(buffer);
    assert_eq!(writer.buf, [1, 2, 3]);
}

#[test]
fn test_writer_new_with_string() {
    struct Writer<B> {
        buf: B,
    }
    
    let buffer: String = String::from("Hello");
    let writer = new(buffer);
    assert_eq!(writer.buf, "Hello");
}

#[test]
fn test_writer_new_with_empty_vec() {
    struct Writer<B> {
        buf: B,
    }
    
    let buffer: Vec<u8> = Vec::new();
    let writer = new(buffer);
    assert_eq!(writer.buf.len(), 0);
}

#[test]
fn test_writer_new_with_large_array() {
    struct Writer<B> {
        buf: B,
    }

    let buffer: [u8; 100] = [0; 100];
    let writer = new(buffer);
    assert_eq!(writer.buf.iter().sum::<u8>(), 0);
}

