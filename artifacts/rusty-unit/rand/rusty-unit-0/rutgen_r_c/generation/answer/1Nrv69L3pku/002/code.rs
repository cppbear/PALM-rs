// Answer 0

#[test]
fn test_sample_var_eq_range() {
    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    let mut rng = TestRng { value: 63 }; // 62 is the max index, 63 should trigger the loop
    let alphanumeric = Alphanumeric;

    // Ensure that calling sample does not panic by iterating
    let result = alphanumeric.sample(&mut rng);
    assert!(result == b'A' || result == b'B' || result == b'C' || result == b'D' ||
            result == b'E' || result == b'F' || result == b'G' || result == b'H' ||
            result == b'I' || result == b'J' || result == b'K' || result == b'L' ||
            result == b'M' || result == b'N' || result == b'O' || result == b'P' ||
            result == b'Q' || result == b'R' || result == b'S' || result == b'T' ||
            result == b'U' || result == b'V' || result == b'W' || result == b'X' ||
            result == b'Y' || result == b'Z' || result == b'a' || result == b'b' ||
            result == b'c' || result == b'd' || result == b'e' || result == b'f' ||
            result == b'g' || result == b'h' || result == b'i' || result == b'j' ||
            result == b'k' || result == b'l' || result == b'm' || result == b'n' ||
            result == b'o' || result == b'p' || result == b'q' || result == b'r' ||
            result == b's' || result == b't' || result == b'u' || result == b'v' ||
            result == b'w' || result == b'x' || result == b'y' || result == b'z' ||
            result == b'0' || result == b'1' || result == b'2' || result == b'3' ||
            result == b'4' || result == b'5' || result == b'6' || result == b'7' ||
            result == b'8' || result == b'9');
}

#[test]
fn test_sample_var_less_than_range() {
    struct TestRng {
        value: u32,
    }

    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
    }

    for value in 0..63 {
        let mut rng = TestRng { value }; // values 0 to 62
        let alphanumeric = Alphanumeric;

        let result = alphanumeric.sample(&mut rng);
        assert!(result == b'A' || result == b'B' || result == b'C' || result == b'D' ||
                result == b'E' || result == b'F' || result == b'G' || result == b'H' ||
                result == b'I' || result == b'J' || result == b'K' || result == b'L' ||
                result == b'M' || result == b'N' || result == b'O' || result == b'P' ||
                result == b'Q' || result == b'R' || result == b'S' || result == b'T' ||
                result == b'U' || result == b'V' || result == b'W' || result == b'X' ||
                result == b'Y' || result == b'Z' || result == b'a' || result == b'b' ||
                result == b'c' || result == b'd' || result == b'e' || result == b'f' ||
                result == b'g' || result == b'h' || result == b'i' || result == b'j' ||
                result == b'k' || result == b'l' || result == b'm' || result == b'n' ||
                result == b'o' || result == b'p' || result == b'q' || result == b'r' ||
                result == b's' || result == b't' || result == b'u' || result == b'v' ||
                result == b'w' || result == b'x' || result == b'y' || result == b'z' ||
                result == b'0' || result == b'1' || result == b'2' || result == b'3' ||
                result == b'4' || result == b'5' || result == b'6' || result == b'7' ||
                result == b'8' || result == b'9');
    }
}

