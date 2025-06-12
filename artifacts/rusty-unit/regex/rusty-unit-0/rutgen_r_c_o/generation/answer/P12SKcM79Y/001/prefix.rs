// Answer 0

#[test]
fn test_build_parser_with_nest_limit_0_octal_false_allow_invalid_utf8_false_ignore_whitespace_false() {
    let parser_builder = ParserBuilder::new()
        .nest_limit(0)
        .octal(false)
        .allow_invalid_utf8(false)
        .ignore_whitespace(false);

    let parser = parser_builder.build();
}

#[test]
fn test_build_parser_with_nest_limit_10_octal_true_allow_invalid_utf8_true_ignore_whitespace_true() {
    let parser_builder = ParserBuilder::new()
        .nest_limit(10)
        .octal(true)
        .allow_invalid_utf8(true)
        .ignore_whitespace(true);

    let parser = parser_builder.build();
}

#[test]
fn test_build_parser_with_nest_limit_5_octal_false_allow_invalid_utf8_true_ignore_whitespace_true() {
    let parser_builder = ParserBuilder::new()
        .nest_limit(5)
        .octal(false)
        .allow_invalid_utf8(true)
        .ignore_whitespace(true);

    let parser = parser_builder.build();
}

#[test]
fn test_build_parser_with_nest_limit_3_octal_true_allow_invalid_utf8_false_ignore_whitespace_false() {
    let parser_builder = ParserBuilder::new()
        .nest_limit(3)
        .octal(true)
        .allow_invalid_utf8(false)
        .ignore_whitespace(false);

    let parser = parser_builder.build();
}

#[test]
fn test_build_parser_with_nest_limit_7_octal_false_allow_invalid_utf8_false_ignore_whitespace_true() {
    let parser_builder = ParserBuilder::new()
        .nest_limit(7)
        .octal(false)
        .allow_invalid_utf8(false)
        .ignore_whitespace(true);

    let parser = parser_builder.build();
}

#[test]
fn test_build_parser_with_nest_limit_9_octal_true_allow_invalid_utf8_true_ignore_whitespace_false() {
    let parser_builder = ParserBuilder::new()
        .nest_limit(9)
        .octal(true)
        .allow_invalid_utf8(true)
        .ignore_whitespace(false);

    let parser = parser_builder.build();
}

#[test]
fn test_build_parser_with_nest_limit_1_octal_true_allow_invalid_utf8_false_ignore_whitespace_true() {
    let parser_builder = ParserBuilder::new()
        .nest_limit(1)
        .octal(true)
        .allow_invalid_utf8(false)
        .ignore_whitespace(true);

    let parser = parser_builder.build();
}

