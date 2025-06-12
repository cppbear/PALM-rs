// Answer 0

#[derive(Debug)]
struct StringSink<'a> {
    string: &'a mut String,
}

#[test]
fn test_new_creates_stringsink() {
    let mut test_string = String::from("initial");
    let sink = new(&mut test_string);
    assert_eq!(sink.string, "initial");
}

