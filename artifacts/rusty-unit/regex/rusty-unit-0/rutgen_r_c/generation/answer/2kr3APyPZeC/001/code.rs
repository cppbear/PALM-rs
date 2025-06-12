// Answer 0

#[test]
fn test_printer_builder_build() {
    // Initialize the PrinterBuilder using its provided method.
    let builder = PrinterBuilder::new();
    
    // Call the build method to generate a Printer instance.
    let printer = builder.build();
    
    // Validate the properties of the Printer instance.
    assert_eq!(std::mem::size_of_val(&printer), std::mem::size_of::<Printer>());
}

