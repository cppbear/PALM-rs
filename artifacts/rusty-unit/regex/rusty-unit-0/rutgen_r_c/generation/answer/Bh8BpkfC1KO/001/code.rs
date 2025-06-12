// Answer 0

#[test]
fn test_printer_builder_build() {
    struct TestPrinterBuilder {
        _priv: (),
    }
    
    impl TestPrinterBuilder {
        fn new() -> TestPrinterBuilder {
            TestPrinterBuilder { _priv: () }
        }
        
        fn build(&self) -> Printer {
            Printer { _priv: () }
        }
    }
    
    let builder = TestPrinterBuilder::new();
    let printer = builder.build();
    
    assert_eq!(std::mem::size_of_val(&printer), std::mem::size_of::<Printer>());
}

#[test]
#[should_panic]
fn test_printer_builder_build_panic() {
    struct InvalidPrinterBuilder;

    impl InvalidPrinterBuilder {
        fn build(&self) -> Printer {
            panic!("This should not succeed");
        }
    }

    let invalid_builder = InvalidPrinterBuilder;
    invalid_builder.build();
}

