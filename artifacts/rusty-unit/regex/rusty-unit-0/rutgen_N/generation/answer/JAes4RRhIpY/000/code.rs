// Answer 0

#[test]
fn test_new_printer_builder() {
    struct PrinterBuilder {
        _priv: (),
    }

    fn new() -> PrinterBuilder {
        PrinterBuilder {
            _priv: (),
        }
    }

    let printer_builder = new();
    assert!(std::mem::size_of_val(&printer_builder) > 0);
}

