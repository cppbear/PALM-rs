// Answer 0

fn cmd_utf8_ranges(args: &Args) -> Result<()> {
    use syntax::ParserBuilder;
    use syntax::hir::{self, HirKind};
    use utf8_ranges::Utf8Sequences;

    let hir = ParserBuilder::new()
        .build()
        .parse(&format!("[{}]", args.arg_class))?;
    let cls = match hir.into_kind() {
        HirKind::Class(hir::Class::Unicode(cls)) => cls,
        _ => return Err(
            format!("unexpected HIR, expected Unicode class").into(),
        ),
    };
    let mut char_count = 0;
    for (i, range) in cls.iter().enumerate() {
        if i > 0 {
            println!("----------------------------");
        }
        char_count += (range.end() as u32) - (range.start() as u32) + 1;
        for seq in Utf8Sequences::new(range.start(), range.end()) {
            for utf8_range in seq.into_iter() {
                print!("[{:02X}-{:02X}]", utf8_range.start, utf8_range.end);
            }
            println!();
        }
    }
    println!("codepoint count: {}", char_count);
    Ok(())
}

struct Args {
    arg_class: String,
}

#[test]
fn test_cmd_utf8_ranges_valid_unicode_class() {
    let args = Args {
        arg_class: "a-z".to_string(),
    };
    let result = cmd_utf8_ranges(&args);
    assert!(result.is_ok());
}

#[test]
fn test_cmd_utf8_ranges_invalid_hir_class() {
    let args = Args {
        arg_class: "123".to_string(),
    };
    let result = cmd_utf8_ranges(&args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "unexpected HIR, expected Unicode class");
}

#[test]
fn test_cmd_utf8_ranges_non_unicode() {
    let args = Args {
        arg_class: "!@#".to_string(),
    };
    let result = cmd_utf8_ranges(&args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "unexpected HIR, expected Unicode class");
}

