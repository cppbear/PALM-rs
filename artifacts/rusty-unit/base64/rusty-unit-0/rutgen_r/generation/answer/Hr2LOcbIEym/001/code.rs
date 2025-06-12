// Answer 0

#[test]
fn test_decoder_new() {
    struct DummyReader;

    impl std::io::Read for DummyReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            buf.fill(0);
            Ok(buf.len())
        }
    }

    struct DummyEngine;

    let reader = DummyReader;
    let engine = &DummyEngine;

    let decoder = base64::new(reader, engine);

    assert_eq!(decoder.b64_buffer, [0; base64::BUF_SIZE]);
    assert_eq!(decoder.b64_offset, 0);
    assert_eq!(decoder.b64_len, 0);
    assert_eq!(decoder.decoded_chunk_buffer, [0; base64::DECODED_CHUNK_SIZE]);
    assert_eq!(decoder.decoded_offset, 0);
    assert_eq!(decoder.decoded_len, 0);
    assert_eq!(decoder.input_consumed_len, 0);
    assert_eq!(decoder.padding_offset, None);
}

