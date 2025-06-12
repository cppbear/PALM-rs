// Answer 0

#[test]
fn test_into_inner_with_vec_writer() {
    let writer = Vec::new();
    let serializer = Serializer::with_formatter(writer, CompactFormatter);
    serializer.into_inner();
}

#[test]
fn test_into_inner_with_buffered_writer() {
    let buffer = std::io::BufWriter::new(Vec::new());
    let serializer = Serializer::with_formatter(buffer, CompactFormatter);
    serializer.into_inner();
}

#[test]
fn test_into_inner_with_empty_writer() {
    let writer = Vec::new();
    let serializer = Serializer::with_formatter(writer, CompactFormatter);
    serializer.into_inner();
}

#[test]
fn test_into_inner_with_file_writer() {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let file_writer = temp_file.as_file();
    let serializer = Serializer::with_formatter(file_writer, CompactFormatter);
    serializer.into_inner();
}

#[test]
fn test_into_inner_with_string_writer() {
    let writer = String::new();
    let serializer = Serializer::with_formatter(writer, CompactFormatter);
    serializer.into_inner();
}

