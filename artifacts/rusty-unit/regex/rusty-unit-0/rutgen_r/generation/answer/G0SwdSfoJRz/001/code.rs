// Answer 0

#[test]
fn test_new_printer() {
    struct Printer;
    
    struct PrinterBuilder;

    impl PrinterBuilder {
        fn new() -> Self {
            PrinterBuilder
        }

        fn build(self) -> Printer {
            Printer
        }
    }

    let printer = {
        let builder = PrinterBuilder::new();
        builder.build()
    };

    assert!(std::mem::size_of::<Printer>() > 0); // Ensuring Printer is not empty
}

