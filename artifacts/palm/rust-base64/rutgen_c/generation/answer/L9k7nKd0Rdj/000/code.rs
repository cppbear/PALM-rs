// Answer 0

#[test]
fn test_chunked_encoder_new() {
    struct TestEngine;
    
    impl Config for TestEngine {}
    impl DecodeEstimate for TestEngine {}

    let engine = TestEngine;
    let encoder = ChunkedEncoder::new(&engine);
    assert_eq!(std::ptr::eq(encoder.engine, &engine), true);
}

#[test]
fn test_chunked_encoder_new_with_different_engine() {
    struct AnotherTestEngine;

    impl Config for AnotherTestEngine {}
    impl DecodeEstimate for AnotherTestEngine {}

    let engine = AnotherTestEngine;
    let encoder = ChunkedEncoder::new(&engine);
    assert_eq!(std::ptr::eq(encoder.engine, &engine), true);
}

