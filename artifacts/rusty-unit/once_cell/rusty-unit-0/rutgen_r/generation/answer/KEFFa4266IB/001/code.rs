// Answer 0

#[test]
fn test_set_with_full_cell() {
    struct FullCell<'a, T> {
        value: Option<&'a T>,
    }

    impl<'a, T> FullCell<'a, T> {
        fn new() -> Self {
            FullCell { value: None }
        }
        
        fn compare_exchange(&self, _: &'a T) -> Result<(), ()> {
            Err(())
        }
      
        fn set(&self, value: &'a T) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let full_cell: FullCell<u32> = FullCell::new();
    let result = full_cell.set(&10);
    assert_eq!(result, Err(()));
}

#[test]
fn test_set_with_different_value() {
    struct FullCell<'a, T> {
        value: Option<&'a T>,
    }

    impl<'a, T> FullCell<'a, T> {
        fn new() -> Self {
            FullCell { value: None }
        }
        
        fn compare_exchange(&self, _: &'a T) -> Result<(), ()> {
            Err(())
        }
      
        fn set(&self, value: &'a T) -> Result<(), ()> {
            match self.compare_exchange(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }

    let full_cell: FullCell<u32> = FullCell::new();
    let result = full_cell.set(&20);
    assert_eq!(result, Err(()));
}

