// Answer 0

#[test]
fn test_to_raw_capacity_valid_cases() {
    let inputs = [0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30]; // All divisible by 3
    for &input in &inputs {
        let expected = input + input / 3;
        let result = to_raw_capacity(input);
        assert_eq!(result, expected);
    }
}

#[test]
#[should_panic(expected = "requested capacity 4294967295 too large: overflow while converting to raw capacity")]
fn test_to_raw_capacity_overflow() {
    let input = usize::MAX; // This input causes overflow
    to_raw_capacity(input);
}

