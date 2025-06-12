// Answer 0

#[test]
fn test_is_unicode_byte_above_127() {
    let byte_literal1 = Literal::Byte(128);
    let byte_literal2 = Literal::Byte(200);
    let byte_literal3 = Literal::Byte(255);

    byte_literal1.is_unicode();
    byte_literal2.is_unicode();
    byte_literal3.is_unicode();
}

