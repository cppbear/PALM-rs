// Answer 0

#[test]
fn test_printer_new() {
    let printer = Printer::new();
    assert_eq!(format!("{:?}", printer), "Printer { _priv: () }");
}

#[test]
fn test_printer_builder_new() {
    let builder = PrinterBuilder::new();
    let printer = builder.build();
    assert_eq!(format!("{:?}", printer), "Printer { _priv: () }");
}

