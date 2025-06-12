// Answer 0

#[derive(Debug)]
struct DecoderReader {
    b64_offset: usize,
    b64_len: usize,
    decoded_chunk_buffer: Vec<u8>,
    decoded_offset: usize,
    decoded_len: usize,
    input_consumed_len: usize,
    padding_offset: usize,
}

#[test]
fn test_decoder_reader_with_valid_data() {
    let decoder = DecoderReader {
        b64_offset: 0,
        b64_len: 10,
        decoded_chunk_buffer: vec![0, 1, 2, 3, 4, 5],
        decoded_offset: 0,
        decoded_len: 6,
        input_consumed_len: 6,
        padding_offset: 0,
    };
    
    let mut output = String::new();
    write!(&mut output, "{:?}", decoder).unwrap();
    
    assert!(output.contains("b64_offset: 0"));
    assert!(output.contains("b64_len: 10"));
    assert!(output.contains("decoded_chunk_buffer: [0, 1, 2, 3, 4, 5]"));
    assert!(output.contains("decoded_offset: 0"));
    assert!(output.contains("decoded_len: 6"));
    assert!(output.contains("input_consumed_len: 6"));
    assert!(output.contains("padding_offset: 0"));
}

#[test]
fn test_decoder_reader_with_empty_buffer() {
    let decoder = DecoderReader {
        b64_offset: 0,
        b64_len: 0,
        decoded_chunk_buffer: Vec::new(),
        decoded_offset: 0,
        decoded_len: 0,
        input_consumed_len: 0,
        padding_offset: 0,
    };
    
    let mut output = String::new();
    write!(&mut output, "{:?}", decoder).unwrap();

    assert!(output.contains("b64_offset: 0"));
    assert!(output.contains("b64_len: 0"));
    assert!(output.contains("decoded_chunk_buffer: []"));
    assert!(output.contains("decoded_offset: 0"));
    assert!(output.contains("decoded_len: 0"));
    assert!(output.contains("input_consumed_len: 0"));
    assert!(output.contains("padding_offset: 0"));
}

#[test]
fn test_decoder_reader_with_large_buffer() {
    let decoder = DecoderReader {
        b64_offset: 100,
        b64_len: 1000,
        decoded_chunk_buffer: vec![0; 1000],
        decoded_offset: 100,
        decoded_len: 900,
        input_consumed_len: 500,
        padding_offset: 50,
    };
    
    let mut output = String::new();
    write!(&mut output, "{:?}", decoder).unwrap();
    
    assert!(output.contains("b64_offset: 100"));
    assert!(output.contains("b64_len: 1000"));
    assert!(output.contains("decoded_chunk_buffer: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]"));
    assert!(output.contains("decoded_offset: 100"));
    assert!(output.contains("decoded_len: 900"));
    assert!(output.contains("input_consumed_len: 500"));
    assert!(output.contains("padding_offset: 50"));
}

