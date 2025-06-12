// Answer 0

#[test]
fn test_pretty_serializer_with_empty_writer() {
    let writer = vec![];
    let serializer = Serializer::pretty(writer);
}

#[test]
fn test_pretty_serializer_with_small_writer() {
    let writer = vec![0u8; 1];
    let serializer = Serializer::pretty(writer);
}

#[test]
fn test_pretty_serializer_with_medium_writer() {
    let writer = vec![0u8; 100];
    let serializer = Serializer::pretty(writer);
}

#[test]
fn test_pretty_serializer_with_large_writer() {
    let writer = vec![0u8; 1_000_000];
    let serializer = Serializer::pretty(writer);
}

#[test]
#[should_panic]
fn test_pretty_serializer_with_too_large_writer() {
    let writer = vec![0u8; 1_000_001];
    let _serializer = Serializer::pretty(writer);
}

