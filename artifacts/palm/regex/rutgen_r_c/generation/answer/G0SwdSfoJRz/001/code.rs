// Answer 0

#[test]
fn test_printer_new() {
    let printer = Printer::new();
    assert!(std::any::TypeId::of::<Printer>() == std::any::TypeId::of_val(&printer));
}

#[test]
fn test_printer_builder_new() {
    let builder = PrinterBuilder::new();
    assert!(std::any::TypeId::of::<PrinterBuilder>() == std::any::TypeId::of_val(&builder));
}

#[test]
fn test_printer_builder_build() {
    let builder = PrinterBuilder::new();
    let printer = builder.build();
    assert!(std::any::TypeId::of::<Printer>() == std::any::TypeId::of_val(&printer));
}

