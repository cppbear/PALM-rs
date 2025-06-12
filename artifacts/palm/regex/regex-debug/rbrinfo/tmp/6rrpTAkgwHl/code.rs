fn parse(re: &str) -> Result<Hir> {
    use syntax::ParserBuilder;
    ParserBuilder::new()
        .allow_invalid_utf8(true)
        .build()
        .parse(re)
        .map_err(From::from)
}