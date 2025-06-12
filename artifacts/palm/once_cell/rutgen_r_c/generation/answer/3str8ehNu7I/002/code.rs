// Answer 0

#[test]
fn test_get_uninitialized() {
    struct TestCell;
    impl TestCell {
        fn is_initialized(&self) -> bool {
            false
        }
        unsafe fn get_unchecked(&self) -> &u32 {
            &0 // This should never be called as is_initialized is false
        }
    }
    
    struct Imp<T>(TestCell);
    
    struct OnceCell<T>(Imp<T>);
    
    impl<T> OnceCell<T> {
        pub const fn new() -> OnceCell<T> {
            OnceCell(Imp(TestCell))
        }
        
        pub fn get(&self) -> Option<&T> {
            if self.0.0.is_initialized() {
                Some(unsafe { self.0.0.get_unchecked() })
            } else {
                None
            }
        }
    }

    let cell: OnceCell<u32> = OnceCell::new();
    assert_eq!(cell.get(), None);
}

