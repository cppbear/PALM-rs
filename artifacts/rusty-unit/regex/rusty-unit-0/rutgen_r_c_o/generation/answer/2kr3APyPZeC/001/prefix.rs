// Answer 0

#[test]
fn test_build_empty_printer() {
    let builder = PrinterBuilder::new();
    let printer = builder.build();
}

#[test]
fn test_build_multiple_printers() {
    let builder1 = PrinterBuilder::new();
    let printer1 = builder1.build();
    
    let builder2 = PrinterBuilder::new();
    let printer2 = builder2.build();
}

