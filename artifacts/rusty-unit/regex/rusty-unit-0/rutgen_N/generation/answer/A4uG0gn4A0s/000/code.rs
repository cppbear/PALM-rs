// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    struct Literal {
        pattern: Vec<u8>,
    }

    impl Literal {
        fn approximate_size(&self) -> usize {
            (self.pattern.len() * mem::size_of::<u8>())
                + (256 * mem::size_of::<usize>()) // skip table
        }
    }

    #[test]
    fn test_approximate_size_empty_pattern() {
        let literal = Literal { pattern: vec![] };
        assert_eq!(literal.approximate_size(), 256 * mem::size_of::<usize>());
    }

    #[test]
    fn test_approximate_size_single_byte_pattern() {
        let literal = Literal { pattern: vec![b'a'] };
        assert_eq!(literal.approximate_size(), (1 * mem::size_of::<u8>()) + (256 * mem::size_of::<usize>()));
    }

    #[test]
    fn test_approximate_size_multiple_bytes_pattern() {
        let pattern = vec![b'a', b'b', b'c', b'd', b'e'];
        let literal = Literal { pattern };
        assert_eq!(literal.approximate_size(), (5 * mem::size_of::<u8>()) + (256 * mem::size_of::<usize>()));
    }
}

