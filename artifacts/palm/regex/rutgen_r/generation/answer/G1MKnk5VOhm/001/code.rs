// Answer 0

#[test]
fn test_new_options_valid_input() {
    struct RegexOptions;

    struct ExecBuilder {
        options: RegexOptions,
        match_type: Option<String>,
        bytes: bool,
        only_utf8: bool,
    }

    fn new_options(opts: RegexOptions) -> ExecBuilder {
        ExecBuilder {
            options: opts,
            match_type: None,
            bytes: false,
            only_utf8: true,
        }
    }

    let opts = RegexOptions;
    let exec_builder = new_options(opts);

    assert_eq!(exec_builder.match_type, None);
    assert_eq!(exec_builder.bytes, false);
    assert_eq!(exec_builder.only_utf8, true);
}

#[test]
#[should_panic]
fn test_new_options_panic_condition() {
    struct RegexOptions;

    struct ExecBuilder {
        options: RegexOptions,
        match_type: Option<String>,
        bytes: bool,
        only_utf8: bool,
    }

    fn new_options(opts: RegexOptions) -> ExecBuilder {
        if false {
            panic!("This condition should never be true");
        }
        ExecBuilder {
            options: opts,
            match_type: None,
            bytes: false,
            only_utf8: true,
        }
    }

    let opts = RegexOptions;
    let _ = new_options(opts);
}

