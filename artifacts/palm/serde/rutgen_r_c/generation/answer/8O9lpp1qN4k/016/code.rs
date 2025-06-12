// Answer 0

#[test]
fn test_unexpected_signed_i16() {
    let content_1 = Content::I16(0);
    let unexpected_1 = content_1.unexpected();
    assert_eq!(unexpected_1, Unexpected::Signed(0));

    let content_2 = Content::I16(-5);
    let unexpected_2 = content_2.unexpected();
    assert_eq!(unexpected_2, Unexpected::Signed(-5));

    let content_3 = Content::I16(100);
    let unexpected_3 = content_3.unexpected();
    assert_eq!(unexpected_3, Unexpected::Signed(100));

    let content_4 = Content::I16(i16::min_value());
    let unexpected_4 = content_4.unexpected();
    assert_eq!(unexpected_4, Unexpected::Signed(i16::min_value() as i64));

    let content_5 = Content::I16(i16::max_value());
    let unexpected_5 = content_5.unexpected();
    assert_eq!(unexpected_5, Unexpected::Signed(i16::max_value() as i64));
}

