// Answer 0

#[test]
fn test_decoded_len_estimate() {
    let encoded_length_1 = 1;
    let encoded_length_2 = 2;
    let encoded_length_3 = 3;
    let encoded_length_4 = 4;
    let encoded_length_5 = 5;

    assert_eq!(3, decoded_len_estimate(encoded_length_1));
    assert_eq!(3, decoded_len_estimate(encoded_length_2));
    assert_eq!(3, decoded_len_estimate(encoded_length_3));
    assert_eq!(3, decoded_len_estimate(encoded_length_4));
    assert_eq!(6, decoded_len_estimate(encoded_length_5));
}

#[test]
fn test_decoded_len_estimate_boundaries() {
    let encoded_length_0 = 0;

    assert_eq!(0, decoded_len_estimate(encoded_length_0));
    assert_eq!(3, decoded_len_estimate(1));
    assert_eq!(3, decoded_len_estimate(2));
    assert_eq!(3, decoded_len_estimate(3));
    assert_eq!(3, decoded_len_estimate(4));
    assert_eq!(6, decoded_len_estimate(5));
}

