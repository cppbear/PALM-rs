// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct Literals;

    struct Teddy;

    #[test]
    fn test_new_returns_none() {
        // Given we have a Literals instance
        let literals_instance = Literals;

        // When we call the new function
        let result = new(&literals_instance);

        // Then we expect the result to be None
        assert_eq!(result, None);
    }
}

