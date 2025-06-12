// Answer 0

#[test]
fn test_new_string_sink_valid() {
    let mut test_string = String::new();
    let sink = StringSink::new(&mut test_string);
    assert_eq!(sink.string.len(), 0);
}

#[test]
fn test_new_string_sink_with_initial_data() {
    let mut test_string = String::from("initial data");
    let sink = StringSink::new(&mut test_string);
    assert_eq!(sink.string, "initial data");
}

#[test]
fn test_new_string_sink_empty_string() {
    let mut test_string = String::new();
    let sink = StringSink::new(&mut test_string);
    assert!(sink.string.is_empty());
}

