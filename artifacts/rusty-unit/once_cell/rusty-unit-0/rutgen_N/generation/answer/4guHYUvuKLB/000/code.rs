// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::unsync::OnceCell;

    #[test]
    fn test_get_mut_on_empty_cell() {
        let mut cell: OnceCell<u32> = OnceCell::new();
        let result = cell.get_mut();
        assert!(result.is_none());
    }

    #[test]
    fn test_get_mut_after_set() {
        let mut cell: OnceCell<u32> = OnceCell::new();
        cell.set(42).unwrap();
        let value = cell.get_mut().unwrap();
        *value = 100;
        assert_eq!(cell.get(), Some(&100));
    }

    #[test]
    #[should_panic]
    fn test_get_mut_panic_on_multiple_mutations() {
        let mut cell: OnceCell<u32> = OnceCell::new();
        cell.set(10).unwrap();
        {
            let _first_mut = cell.get_mut().unwrap();
            // Attempt to re-borrow mutably
            let _second_mut = cell.get_mut().unwrap(); // This should panic
        }
    }

    #[test]
    fn test_get_mut_after_double_set() {
        let mut cell: OnceCell<u32> = OnceCell::new();
        cell.set(20).unwrap();
        let _value = cell.get_mut().unwrap();
        cell.set(30).unwrap();
        assert_eq!(cell.get(), Some(&30));
    }
}

