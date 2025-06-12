// Answer 0

#[test]
fn test_peek_end_of_value_with_space() {
    let read = Some(b' ');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_with_newline() {
    let read = Some(b'\n');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_with_tab() {
    let read = Some(b'\t');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_with_carriage_return() {
    let read = Some(b'\r');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_with_double_quote() {
    let read = Some(b'"');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_with_open_bracket() {
    let read = Some(b'[');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_with_close_bracket() {
    let read = Some(b']');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_with_open_brace() {
    let read = Some(b'{');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_with_close_brace() {
    let read = Some(b'}');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_with_comma() {
    let read = Some(b',');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_with_colon() {
    let read = Some(b':');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
fn test_peek_end_of_value_with_none() {
    let read = None;
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

#[test]
#[should_panic]
fn test_peek_end_of_value_with_invalid_byte() {
    let read = Some(b'x');
    let deserializer = Deserializer::new(read);
    let mut stream_deserializer = StreamDeserializer::new(deserializer);
    stream_deserializer.peek_end_of_value();
}

