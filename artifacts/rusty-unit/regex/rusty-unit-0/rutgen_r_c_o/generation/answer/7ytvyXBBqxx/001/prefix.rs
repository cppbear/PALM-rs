// Answer 0

#[test]
fn test_printer_new() {
    let printer = Printer::new();
}

#[test]
fn test_printer_builder_new() {
    let builder = PrinterBuilder::new();
    let printer = builder.build();
}

