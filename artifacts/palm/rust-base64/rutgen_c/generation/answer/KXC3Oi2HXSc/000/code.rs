// Answer 0

#[test]
fn test_new_with_zero_length() {
    let estimate = GeneralPurposeEstimate::new(0);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 0);
}

#[test]
fn test_new_with_length_one() {
    let estimate = GeneralPurposeEstimate::new(1);
    assert_eq!(estimate.rem, 1);
    assert_eq!(estimate.conservative_decoded_len, 3);
}

#[test]
fn test_new_with_length_two() {
    let estimate = GeneralPurposeEstimate::new(2);
    assert_eq!(estimate.rem, 2);
    assert_eq!(estimate.conservative_decoded_len, 3);
}

#[test]
fn test_new_with_length_three() {
    let estimate = GeneralPurposeEstimate::new(3);
    assert_eq!(estimate.rem, 3);
    assert_eq!(estimate.conservative_decoded_len, 3);
}

#[test]
fn test_new_with_multiple_of_four() {
    let estimate = GeneralPurposeEstimate::new(4);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 3);
}

#[test]
fn test_new_with_length_five() {
    let estimate = GeneralPurposeEstimate::new(5);
    assert_eq!(estimate.rem, 1);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_new_with_length_six() {
    let estimate = GeneralPurposeEstimate::new(6);
    assert_eq!(estimate.rem, 2);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_new_with_length_seven() {
    let estimate = GeneralPurposeEstimate::new(7);
    assert_eq!(estimate.rem, 3);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_new_with_length_eight() {
    let estimate = GeneralPurposeEstimate::new(8);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

