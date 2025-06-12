// Answer 0

#[test]
fn test_printer_builder_new() {
    struct PrinterBuilder {
        _priv: (),
    }

    impl PrinterBuilder {
        fn new() -> PrinterBuilder {
            PrinterBuilder { _priv: () }
        }
    }

    let printer_builder = PrinterBuilder::new();
    assert_eq!(std::mem::size_of_val(&printer_builder), std::mem::size_of::<PrinterBuilder>());
}

