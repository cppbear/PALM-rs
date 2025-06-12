// Answer 0

#[test]
fn test_case_insensitive_true() {
    let mut builder = ParserBuilder::new();
    builder.hir.allow_invalid_utf8(true);
    builder.nest_limit(100);
    builder.ignore_whitespace(false);
    builder.octal(true);
    builder.case_insensitive(true);
}

#[test]
fn test_case_insensitive_false() {
    let mut builder = ParserBuilder::new();
    builder.hir.allow_invalid_utf8(false);
    builder.nest_limit(50);
    builder.ignore_whitespace(true);
    builder.octal(false);
    builder.case_insensitive(false);
}

#[test]
fn test_case_insensitive_edge_case() {
    let mut builder = ParserBuilder::new();
    builder.hir.allow_invalid_utf8(true);
    builder.nest_limit(0);
    builder.ignore_whitespace(false);
    builder.octal(true);
    builder.case_insensitive(true);
}

#[test]
fn test_case_insensitive_max_nest_limit() {
    let mut builder = ParserBuilder::new();
    builder.hir.allow_invalid_utf8(false);
    builder.nest_limit(u32::MAX);
    builder.ignore_whitespace(true);
    builder.octal(false);
    builder.case_insensitive(false);
}

#[test]
fn test_case_insensitive_min_nest_limit() {
    let mut builder = ParserBuilder::new();
    builder.hir.allow_invalid_utf8(true);
    builder.nest_limit(0);
    builder.ignore_whitespace(false);
    builder.octal(true);
    builder.case_insensitive(false);
}

