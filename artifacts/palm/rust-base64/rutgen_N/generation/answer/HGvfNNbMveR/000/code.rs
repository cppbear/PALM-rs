// Answer 0

#[test]
fn test_decoder_reader_fmt() {
    use std::fmt;

    struct DecoderReader {
        b64_offset: usize,
        b64_len: usize,
        decoded_chunk_buffer: Vec<u8>,
        decoded_offset: usize,
        decoded_len: usize,
        input_consumed_len: usize,
        padding_offset: usize,
    }

    impl fmt::Debug for DecoderReader {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("DecoderReader")
                .field("b64_offset", &self.b64_offset)
                .field("b64_len", &self.b64_len)
                .field("decoded_chunk_buffer", &self.decoded_chunk_buffer)
                .field("decoded_offset", &self.decoded_offset)
                .field("decoded_len", &self.decoded_len)
                .field("input_consumed_len", &self.input_consumed_len)
                .field("padding_offset", &self.padding_offset)
                .finish()
        }
    }

    let decoder = DecoderReader {
        b64_offset: 10,
        b64_len: 20,
        decoded_chunk_buffer: vec![1, 2, 3],
        decoded_offset: 5,
        decoded_len: 15,
        input_consumed_len: 30,
        padding_offset: 0,
    };

    let expected = r#"DecoderReader { b64_offset: 10, b64_len: 20, decoded_chunk_buffer: [1, 2, 3], decoded_offset: 5, decoded_len: 15, input_consumed_len: 30, padding_offset: 0 }"#;

    let mut output = Vec::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        decoder.fmt(&mut formatter).unwrap();
    }

    let output_str = String::from_utf8(output).unwrap();
    assert_eq!(output_str, expected);
}

