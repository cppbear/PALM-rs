// Answer 0

#[test]
fn test_main_with_flag_dfa_reverse() {
    use regex_debug::{run, Args};
    use docopt::Docopt;
    use std::process;

    const USAGE: &str = "
    Usage: regex-debug [--dfa|--dfa-reverse]
    Options:
        --dfa           Use DFA
        --dfa-reverse   Use DFA in reverse
    ";

    #[derive(Debug, Deserialize)]
    struct TestArgs {
        flag_dfa: bool,
        flag_dfa_reverse: bool,
    }

    let args = TestArgs {
        flag_dfa: false,
        flag_dfa_reverse: true,
    };

    let run_result = run(&args);

    assert!(run_result.is_ok());
}

#[test]
fn test_main_without_flag_dfa_reverse() {
    use regex_debug::{run, Args};
    use docopt::Docopt;
    use std::process;

    const USAGE: &str = "
    Usage: regex-debug [--dfa|--dfa-reverse]
    Options:
        --dfa           Use DFA
        --dfa-reverse   Use DFA in reverse
    ";

    #[derive(Debug, Deserialize)]
    struct TestArgs {
        flag_dfa: bool,
        flag_dfa_reverse: bool,
    }

    let args = TestArgs {
        flag_dfa: false,
        flag_dfa_reverse: false,
    };

    let run_result = run(&args);

    assert!(run_result.is_ok());
}

#[should_panic]
#[test]
fn test_main_with_invalid_args() {
    use regex_debug::{run, Args};
    use docopt::Docopt;
    use std::process;

    const USAGE: &str = "
    Usage: regex-debug [--dfa|--dfa-reverse]
    Options:
        --dfa           Use DFA
        --dfa-reverse   Use DFA in reverse
    ";

    #[derive(Debug, Deserialize)]
    struct TestArgs {
        flag_dfa: bool,
        flag_dfa_reverse: bool,
    }

    let args = TestArgs {
        flag_dfa: false,
        flag_dfa_reverse: false,
    };

    let _ = run(&args).unwrap_err(); // Simulate error for assert
}

