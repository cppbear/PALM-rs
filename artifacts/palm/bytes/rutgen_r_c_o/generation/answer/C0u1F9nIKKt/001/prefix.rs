// Answer 0

#[derive(Debug)]
struct TestBuf {
    size: usize,
}

impl Buf for TestBuf {
    // Implementation details here, as needed for tests
}

#[test]
fn test_new_reader_empty_buffer() {
    let buf = TestBuf { size: 0 };
    let reader = new(buf);
}

#[test]
fn test_new_reader_small_buffer() {
    let buf = TestBuf { size: 1 };
    let reader = new(buf);
}

#[test]
fn test_new_reader_medium_buffer() {
    let buf = TestBuf { size: 1024 }; // 1 KiB
    let reader = new(buf);
}

#[test]
fn test_new_reader_large_buffer() {
    let buf = TestBuf { size: 1048576 }; // 1 MiB
    let reader = new(buf);
}

#[test]
fn test_new_reader_full_buffer() {
    let buf = TestBuf { size: 8192 }; // 8 KiB
    let reader = new(buf);
}

