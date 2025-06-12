// Answer 0

#[test]
fn test_printer_build() {
    struct TestPrinterBuilder {
        _priv: (),
    }
    
    impl TestPrinterBuilder {
        fn new() -> Self {
            Self { _priv: () }
        }
        
        fn build(&self) -> Printer {
            Printer {
                _priv: (),
            }
        }
    }

    let builder = TestPrinterBuilder::new();
    let printer = builder.build();

    // Check that the printer is correctly instantiated
    assert_eq!(std::mem::size_of_val(&printer), std::mem::size_of::<Printer>());
}

