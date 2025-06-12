// Answer 0

#[test]
fn test_chunked_encoder_new() {
    struct DummyEngine;

    struct ChunkedEncoder<'e, E> {
        engine: &'e E,
    }

    let engine = DummyEngine;
    let encoder = ChunkedEncoder::new(&engine);
    
    assert_eq!(encoder.engine as *const _, &engine as *const _);
}

