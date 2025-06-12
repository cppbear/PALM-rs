// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct ByteContainer {
        data: Vec<u8>,
    }

    impl ByteContainer {
        fn new(data: Vec<u8>) -> Self {
            ByteContainer { data }
        }

        fn as_slice(&self) -> &[u8] {
            &self.data
        }

        fn borrow(&self) -> &[u8] {
            self.as_slice()
        }
    }

    #[test]
    fn test_borrow_empty() {
        let container = ByteContainer::new(vec![]);
        let borrowed = container.borrow();
        assert_eq!(borrowed, &[]);
    }

    #[test]
    fn test_borrow_non_empty() {
        let container = ByteContainer::new(vec![1, 2, 3]);
        let borrowed = container.borrow();
        assert_eq!(borrowed, &[1, 2, 3]);
    }

    #[test]
    fn test_borrow_large_data() {
        let container = ByteContainer::new(vec![0; 1000]);
        let borrowed = container.borrow();
        assert_eq!(borrowed.len(), 1000);
    }
}

