// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::unsync::OnceCell;

    #[test]
    fn test_into_inner_empty() {
        let cell: OnceCell<String> = OnceCell::new();
        assert_eq!(cell.into_inner(), None);
    }

    #[test]
    fn test_into_inner_with_value() {
        let mut cell = OnceCell::new();
        cell.set("hello".to_string()).unwrap();
        assert_eq!(cell.into_inner(), Some("hello".to_string()));
    }

    #[test]
    fn test_into_inner_after_set() {
        let mut cell = OnceCell::new();
        cell.set("world".to_string()).unwrap();
        assert_eq!(cell.into_inner(), Some("world".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_into_inner_after_second_set() {
        let mut cell = OnceCell::new();
        cell.set("first".to_string()).unwrap();
        cell.set("second".to_string()).unwrap(); // This should panic since a value is already set
    }
}

