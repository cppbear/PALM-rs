// Answer 0

#[test]
fn test_printer_builder_new() {
    let builder = PrinterBuilder::new();
    assert_eq!(std::mem::size_of_val(&builder), std::mem::size_of::<PrinterBuilder>());
}

#[test]
fn test_printer_builder_create() {
    let builder = PrinterBuilder::new();
    let printer = builder.build();
    assert!(std::mem::size_of_val(&printer) > 0); // Assuming Printer has some size
}

