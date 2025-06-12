// Answer 0

#[test]
fn test_get_when_empty() {
    struct SimpleCell;
    impl SimpleCell {
        fn is_initialized(&self) -> bool {
            false
        }
        unsafe fn get_unchecked(&self) -> &u32 {
            &0 // Dummy reference, won't be called since is_initialized is false
        }
    }
    
    let cell = OnceCell(SimpleCell);
    assert_eq!(cell.get(), None);
}

#[test]
fn test_get_when_initialized() {
    struct InitializedCell;
    impl InitializedCell {
        fn is_initialized(&self) -> bool {
            true
        }
        unsafe fn get_unchecked(&self) -> &u32 {
            &42 // Returning a dummy value
        }
    }

    let cell = OnceCell(InitializedCell);
    assert_eq!(cell.get(), Some(&42));
}

