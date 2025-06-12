// Answer 0

#[test]
fn test_new_printer_builder() {
    let builder = new();
    assert_eq!(std::mem::discriminant(&builder), std::mem::discriminant(&PrinterBuilder { _priv: () }));
}

struct PrinterBuilder {
    _priv: (),
}

fn new() -> PrinterBuilder {
    PrinterBuilder {
        _priv: (),
    }
}

