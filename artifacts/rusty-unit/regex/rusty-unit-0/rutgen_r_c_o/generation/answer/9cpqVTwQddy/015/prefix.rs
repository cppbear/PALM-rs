// Answer 0

#[test]
fn test_fmt_class_ascii_graph_negated() {
    let mut output = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let ast = ClassAscii {
        span: Span, // Assume Span is defined properly in the ast module
        kind: ClassAsciiKind::Graph,
        negated: true,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_graph_not_negated() {
    let mut output = String::new();
    let writer = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut output,
    };
    let ast = ClassAscii {
        span: Span, // Assume Span is defined properly in the ast module
        kind: ClassAsciiKind::Graph,
        negated: false,
    };
    writer.fmt_class_ascii(&ast);
}

