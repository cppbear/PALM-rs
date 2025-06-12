// Answer 0

#[test]
fn test_generic_hamming_different_length_args_1() {
    let input_a = vec![1, 2, 3];
    let input_b = vec![1, 2];
    generic_hamming(input_a, input_b);
}

#[test]
fn test_generic_hamming_different_length_args_2() {
    let input_a = "abcde".chars();
    let input_b = "abc".chars();
    generic_hamming(input_a, input_b);
}

#[test]
fn test_generic_hamming_different_length_args_3() {
    let input_a = [1, 2, 3, 4];
    let input_b = [1, 2, 3];
    generic_hamming(input_a, input_b);
}

#[test]
fn test_generic_hamming_different_length_args_4() {
    let input_a = vec![true, false];
    let input_b = vec![true];
    generic_hamming(input_a, input_b);
}

#[test]
fn test_generic_hamming_different_length_args_5() {
    let input_a = vec![1, 2, 3, 4, 5];
    let input_b = vec![1, 2];
    generic_hamming(input_a, input_b);
}

