// Answer 0

#[test]
fn test_finish_success() {
    struct Dummy;
    
    impl Dummy {
        fn finish(self) -> Result<()> {
            Ok(())
        }
    }

    let instance = Dummy;
    let result = instance.finish();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_finish_panic() {
    struct Dummy;
    
    impl Dummy {
        fn finish(self) -> Result<()> {
            panic!("Intentional panic for testing");
        }
    }

    let instance = Dummy;
    instance.finish();
}

