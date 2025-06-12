// Answer 0

#[derive(Debug)]
pub enum StrSimError {
    DifferentLengthArgs,
}

pub type HammingResult = Result<usize, StrSimError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_hamming_equal_length_same() {
        let result = generic_hamming(vec![1, 2, 3], vec![1, 2, 3]);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_generic_hamming_equal_length_different() {
        let result = generic_hamming(vec![1, 2, 3], vec![1, 2, 4]);
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn test_generic_hamming_different_length() {
        let result = generic_hamming(vec![1, 2], vec![1, 2, 3]);
        assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
    }

    #[test]
    fn test_generic_hamming_empty_sequences() {
        let result = generic_hamming(vec![], vec![]);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_generic_hamming_one_empty() {
        let result = generic_hamming(vec![1], vec![]);
        assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
    }
}

