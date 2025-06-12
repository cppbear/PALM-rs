// Answer 0

#[test]
fn test_swap_greed_enabled() {
    let mut builder = ParserBuilder::new();
    builder.swap_greed(true);
}

#[test]
fn test_swap_greed_disabled() {
    let mut builder = ParserBuilder::new();
    builder.swap_greed(false);
}

#[test]
fn test_swap_greed_multiple_calls() {
    let mut builder = ParserBuilder::new();
    builder.swap_greed(true);
    builder.swap_greed(false);
    builder.swap_greed(true);
}

#[test]
fn test_swap_greed_with_other_options() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(1);
    builder.octal(true);
    builder.allow_invalid_utf8(true);
    builder.ignore_whitespace(false);
    builder.case_insensitive(true);
    builder.multi_line(false);
    builder.dot_matches_new_line(true);
    builder.swap_greed(true);
}

#[test]
fn test_swap_greed_with_edge_nest_limit() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(0);
    builder.swap_greed(true);
}

#[test]
fn test_swap_greed_with_max_nest_limit() {
    let mut builder = ParserBuilder::new();
    builder.nest_limit(u32::MAX);
    builder.swap_greed(false);
}

