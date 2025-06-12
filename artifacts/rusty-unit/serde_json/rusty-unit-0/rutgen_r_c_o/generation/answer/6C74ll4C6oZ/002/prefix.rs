// Answer 0

#[test]
fn test_parse_index_with_leading_zeros() {
    parse_index("00");
}

#[test]
fn test_parse_index_with_leading_zeros_1() {
    parse_index("01");
}

#[test]
fn test_parse_index_with_leading_zeros_2() {
    parse_index("02");
}

#[test]
fn test_parse_index_with_leading_zeros_3() {
    parse_index("03");
}

#[test]
fn test_parse_index_with_leading_zeros_4() {
    parse_index("04");
}

#[test]
fn test_parse_index_with_leading_zeros_5() {
    parse_index("05");
}

#[test]
fn test_parse_index_with_leading_zeros_6() {
    parse_index("06");
}

#[test]
fn test_parse_index_with_leading_zeros_7() {
    parse_index("07");
}

#[test]
fn test_parse_index_with_leading_zeros_8() {
    parse_index("08");
}

#[test]
fn test_parse_index_with_leading_zeros_9() {
    parse_index("09");
}

#[test]
fn test_parse_index_with_triple_leading_zeros() {
    parse_index("000");
}

#[test]
fn test_parse_index_with_leading_zero_and_other_digits() {
    parse_index("012");
}

#[test]
fn test_parse_index_with_leading_zero_and_other_digits_1() {
    parse_index("013");
}

#[test]
fn test_parse_index_with_non_zero_leading_digits() {
    parse_index("100");
}

#[test]
fn test_parse_index_with_non_zero_leading_digits_1() {
    parse_index("200");
}

