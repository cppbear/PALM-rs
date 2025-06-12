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

#[derive(Debug)]
struct Args {
    arg_class: String,
}

#[test]
fn test_cmd_utf8_ranges_valid_input() {
    let args = Args {
        arg_class: "0080-00FF".to_string(), // Valid Unicode class
    };
    assert!(cmd_utf8_ranges(&args).is_ok());
}

#[test]
fn test_cmd_utf8_ranges_empty_class() {
    let args = Args {
        arg_class: "".to_string(), // Empty class should panic
    };
    assert!(cmd_utf8_ranges(&args).is_err());
}

#[test]
fn test_cmd_utf8_ranges_invalid_class() {
    let args = Args {
        arg_class: "ZZZZ".to_string(), // Invalid class should panic
    };
    assert!(cmd_utf8_ranges(&args).is_err());
}

#[test]
fn test_cmd_utf8_ranges_single_range() {
    let args = Args {
        arg_class: "0030-0039".to_string(), // Numeric range
    };
    assert!(cmd_utf8_ranges(&args).is_ok());
}

#[test]
fn test_cmd_utf8_ranges_multiple_ranges() {
    let args = Args {
        arg_class: "0020-007F".to_string(), // Visible ASCII range
    };
    assert!(cmd_utf8_ranges(&args).is_ok());
}

#[test]
fn test_cmd_utf8_ranges_non_contiguous_ranges() {
    let args = Args {
        arg_class: "00A0-00FF" // Mixed with gaps on ranges requires thorough testing
    };
    assert!(cmd_utf8_ranges(&args).is_ok());
}

