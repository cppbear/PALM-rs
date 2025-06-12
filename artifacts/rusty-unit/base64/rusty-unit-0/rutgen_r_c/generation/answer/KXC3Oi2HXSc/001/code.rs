// Answer 0

#[test]
fn test_general_purpose_estimate_new_zero() {
    let estimate = GeneralPurposeEstimate::new(0);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 0);
}

#[test]
fn test_general_purpose_estimate_new_one() {
    let estimate = GeneralPurposeEstimate::new(1);
    assert_eq!(estimate.rem, 1);
    assert_eq!(estimate.conservative_decoded_len, 3);
}

#[test]
fn test_general_purpose_estimate_new_two() {
    let estimate = GeneralPurposeEstimate::new(2);
    assert_eq!(estimate.rem, 2);
    assert_eq!(estimate.conservative_decoded_len, 3);
}

#[test]
fn test_general_purpose_estimate_new_three() {
    let estimate = GeneralPurposeEstimate::new(3);
    assert_eq!(estimate.rem, 3);
    assert_eq!(estimate.conservative_decoded_len, 3);
}

#[test]
fn test_general_purpose_estimate_new_four() {
    let estimate = GeneralPurposeEstimate::new(4);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 3);
}

#[test]
fn test_general_purpose_estimate_new_five() {
    let estimate = GeneralPurposeEstimate::new(5);
    assert_eq!(estimate.rem, 1);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_general_purpose_estimate_new_six() {
    let estimate = GeneralPurposeEstimate::new(6);
    assert_eq!(estimate.rem, 2);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_general_purpose_estimate_new_seven() {
    let estimate = GeneralPurposeEstimate::new(7);
    assert_eq!(estimate.rem, 3);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_general_purpose_estimate_new_eight() {
    let estimate = GeneralPurposeEstimate::new(8);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 6);
}

#[test]
fn test_general_purpose_estimate_new_nine() {
    let estimate = GeneralPurposeEstimate::new(9);
    assert_eq!(estimate.rem, 1);
    assert_eq!(estimate.conservative_decoded_len, 9);
}

#[test]
fn test_general_purpose_estimate_new_ten() {
    let estimate = GeneralPurposeEstimate::new(10);
    assert_eq!(estimate.rem, 2);
    assert_eq!(estimate.conservative_decoded_len, 9);
}

#[test]
fn test_general_purpose_estimate_new_eleven() {
    let estimate = GeneralPurposeEstimate::new(11);
    assert_eq!(estimate.rem, 3);
    assert_eq!(estimate.conservative_decoded_len, 9);
}

#[test]
fn test_general_purpose_estimate_new_twelve() {
    let estimate = GeneralPurposeEstimate::new(12);
    assert_eq!(estimate.rem, 0);
    assert_eq!(estimate.conservative_decoded_len, 9);
}

