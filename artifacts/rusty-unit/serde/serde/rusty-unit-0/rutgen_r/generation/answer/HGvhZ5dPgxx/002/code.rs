// Answer 0

#[test]
fn test_cautious_with_zero_size_element() {
    struct ZeroSize;

    let result = cautious::<ZeroSize>(Some(10));
    assert_eq!(result, 0);

    let result_none = cautious::<ZeroSize>(None);
    assert_eq!(result_none, 0);
}

