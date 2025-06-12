// Answer 0

#[test]
fn test_begin_object_writer_empty() {
    let mut vec = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: true,
        indent: b"  "
    };
    formatter.begin_object(&mut vec).unwrap();
}

#[test]
fn test_begin_object_writer_non_empty() {
    let mut vec = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 1,
        has_value: true,
        indent: b"  "
    };
    formatter.begin_object(&mut vec).unwrap();
}

#[test]
fn test_begin_object_writer_large() {
    let mut vec = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 4096,
        has_value: false,
        indent: b"  "
    };
    formatter.begin_object(&mut vec).unwrap();
}

#[test]
fn test_begin_object_writer_with_value() {
    let mut vec = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  "
    };
    formatter.begin_object(&mut vec).unwrap();
}

#[test]
fn test_begin_object_writer_with_different_indent() {
    let mut vec = Vec::new();
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: true,
        indent: b"\t"
    };
    formatter.begin_object(&mut vec).unwrap();
}

