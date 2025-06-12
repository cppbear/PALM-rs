// Answer 0

#[test]
fn test_fmt_class_ascii_positive() {
    use std::fmt::Write; // Import the Write trait for the write_str method
    use ast::{ClassAscii, ClassAsciiKind};

    // Prepare a buffer to write to
    let mut buffer = String::new();
    let printer = Printer { _priv: () }; // Create a Printer instance

    // Construct a Writer with the printer and the buffer
    let mut writer = Writer {
        printer: &mut printer,
        wtr: buffer,
    };

    // Create an instance of ClassAscii with the Print kind and negated as false
    let class_ascii = ClassAscii {
        span: Span::default(), // Assuming Span has a default implementation
        kind: ClassAsciiKind::Print,
        negated: false,
    };

    // Call the fmt_class_ascii method and expect it to write "[:print:]"
    writer.fmt_class_ascii(&class_ascii).expect("Failed to write class ascii");
    assert_eq!(writer.wtr, "[:print:]");
}

#[test]
fn test_fmt_class_ascii_negated() {
    use std::fmt::Write; // Import the Write trait for the write_str method
    use ast::{ClassAscii, ClassAsciiKind};

    // Prepare a buffer to write to
    let mut buffer = String::new();
    let printer = Printer { _priv: () }; // Create a Printer instance

    // Construct a Writer with the printer and the buffer
    let mut writer = Writer {
        printer: &mut printer,
        wtr: buffer,
    };

    // Create an instance of ClassAscii with the Print kind and negated as true
    let class_ascii = ClassAscii {
        span: Span::default(), // Assuming Span has a default implementation
        kind: ClassAsciiKind::Print,
        negated: true,
    };

    // Call the fmt_class_ascii method and expect it to write "[:^print:]"
    writer.fmt_class_ascii(&class_ascii).expect("Failed to write class ascii");
    assert_eq!(writer.wtr, "[:^print:]");
}

