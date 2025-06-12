// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct Byte(u16);

    impl Byte {
        fn byte(b: u8) -> Self {
            Byte(b as u16)
        }
    }

    #[test]
    fn test_byte_conversion() {
        let input: u8 = 255;
        let expected = Byte(255);
        let result = Byte::byte(input);
        assert_eq!(result.0, expected.0);
    }

    #[test]
    fn test_byte_min_value() {
        let input: u8 = 0;
        let expected = Byte(0);
        let result = Byte::byte(input);
        assert_eq!(result.0, expected.0);
    }

    #[test]
    fn test_byte_boundary_value() {
        let input: u8 = 128;
        let expected = Byte(128);
        let result = Byte::byte(input);
        assert_eq!(result.0, expected.0);
    }
}

