// Answer 0

#[test]
fn test_write_encoded_bytes_empty() {
    let mut buffer = String::new();
    let mut formatter_sink = FormatterSink { f: &mut fmt::Formatter::new(&mut buffer) };
    let encoded: &[u8] = &[];
    formatter_sink.write_encoded_bytes(encoded);
}

#[test]
fn test_write_encoded_bytes_valid_utf8() {
    let mut buffer = String::new();
    let mut formatter_sink = FormatterSink { f: &mut fmt::Formatter::new(&mut buffer) };
    let encoded: &[u8] = &[97, 98, 99];
    formatter_sink.write_encoded_bytes(encoded);
}

#[test]
#[should_panic(expected = "base64 data was not utf8")]
fn test_write_encoded_bytes_invalid_utf8() {
    let mut buffer = String::new();
    let mut formatter_sink = FormatterSink { f: &mut fmt::Formatter::new(&mut buffer) };
    let encoded: &[u8] = &[255];
    formatter_sink.write_encoded_bytes(encoded);
}

#[test]
fn test_write_encoded_bytes_single_byte() {
    let mut buffer = String::new();
    let mut formatter_sink = FormatterSink { f: &mut fmt::Formatter::new(&mut buffer) };
    let encoded: &[u8] = &[100];
    formatter_sink.write_encoded_bytes(encoded);
}

#[test]
fn test_write_encoded_bytes_multi_byte() {
    let mut buffer = String::new();
    let mut formatter_sink = FormatterSink { f: &mut fmt::Formatter::new(&mut buffer) };
    let encoded: &[u8] = &[104, 101, 108, 108, 111];
    formatter_sink.write_encoded_bytes(encoded);
}

#[test]
fn test_write_encoded_bytes_high_value_byte() {
    let mut buffer = String::new();
    let mut formatter_sink = FormatterSink { f: &mut fmt::Formatter::new(&mut buffer) };
    let encoded: &[u8] = &[240, 159, 146, 150];
    formatter_sink.write_encoded_bytes(encoded);
}

