// Answer 0

#[test]
fn test_put_with_empty_buf() {
    let mut buffer: Vec<u8> = Vec::new();
    let src: Vec<u8> = Vec::new();
    unsafe {
        buffer.put(src);
    }
}

#[test]
fn test_put_with_single_byte_buf() {
    let mut buffer: Vec<u8> = Vec::new();
    let src: Vec<u8> = vec![1];
    unsafe {
        buffer.put(src);
    }
}

#[test]
fn test_put_with_multiple_bytes_buf() {
    let mut buffer: Vec<u8> = Vec::new();
    let src: Vec<u8> = vec![1, 2, 3, 4, 5];
    unsafe {
        buffer.put(src);
    }
}

#[test]
fn test_put_with_large_buf() {
    let mut buffer: Vec<u8> = Vec::new();
    let src: Vec<u8> = (0..1000).map(|x| x as u8).collect();
    unsafe {
        buffer.put(src);
    }
}

