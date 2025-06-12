// Answer 0

#[test]
fn test_writer_new() {
    struct TestBuffer;

    struct Writer<B> {
        buf: B,
    }

    let buffer = TestBuffer;
    let writer = new(buffer);
    assert_eq!(std::mem::size_of::<Writer<TestBuffer>>(), std::mem::size_of::<Writer<TestBuffer>>());
}

#[test]
fn test_writer_new_with_different_buffer_types() {
    struct TestBufferA;
    struct TestBufferB;

    struct Writer<B> {
        buf: B,
    }

    let buffer_a = TestBufferA;
    let writer_a = new(buffer_a);

    let buffer_b = TestBufferB;
    let writer_b = new(buffer_b);

    assert_ne!(std::mem::discriminant(&writer_a), std::mem::discriminant(&writer_b));
}

