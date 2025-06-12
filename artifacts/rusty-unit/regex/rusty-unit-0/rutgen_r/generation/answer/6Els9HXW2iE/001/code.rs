// Answer 0

fn cmd_hir(args: &Args) -> Result<()> {
    use syntax::ParserBuilder;

    let mut parser = ParserBuilder::new()
        .allow_invalid_utf8(false)
        .build();
    let hir = parser.parse(&args.arg_pattern)?;
    println!("{:#?}", hir);
    Ok(())
}

struct Args {
    arg_pattern: String,
}

#[test]
fn test_cmd_hir_valid_pattern() {
    let args = Args { arg_pattern: r"\d+" .to_string() };
    assert!(cmd_hir(&args).is_ok());
}

#[test]
fn test_cmd_hir_invalid_pattern() {
    let args = Args { arg_pattern: r"[".to_string() }; // unclosed bracket
    assert!(cmd_hir(&args).is_err());
}

#[test]
fn test_cmd_hir_empty_pattern() {
    let args = Args { arg_pattern: "".to_string() }; // empty pattern
    assert!(cmd_hir(&args).is_err());
}

#[test]
fn test_cmd_hir_invalid_utf8() {
    let args = Args { arg_pattern: String::from_utf8_lossy(&[0, 159, 146, 150]).into_owned() }; // invalid UTF-8 bytes
    assert!(cmd_hir(&args).is_err());
}

#[test]
fn test_cmd_hir_special_characters() {
    let args = Args { arg_pattern: r"\\.*" .to_string() }; // valid regex with special characters
    assert!(cmd_hir(&args).is_ok());
}

