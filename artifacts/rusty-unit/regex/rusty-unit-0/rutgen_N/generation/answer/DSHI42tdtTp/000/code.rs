// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct Literal {
        v: Vec<u8>,
        cut: bool,
    }

    #[test]
    fn test_new_empty_bytes() {
        let bytes = Vec::new();
        let literal = new(bytes);
        assert_eq!(literal.v.len(), 0);
        assert!(!literal.cut);
    }

    #[test]
    fn test_new_single_byte() {
        let bytes = vec![b'a'];
        let literal = new(bytes);
        assert_eq!(literal.v, vec![b'a']);
        assert!(!literal.cut);
    }

    #[test]
    fn test_new_multiple_bytes() {
        let bytes = vec![b'a', b'b', b'c'];
        let literal = new(bytes);
        assert_eq!(literal.v, vec![b'a', b'b', b'c']);
        assert!(!literal.cut);
    }
}

