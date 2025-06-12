// Answer 0

#[test]
fn test_build_empty_patterns() {
    let exec_builder = ExecBuilder::new_many(Vec::<&str>::new());
    let result = exec_builder.build();
    assert!(result.is_ok());
    let exec = result.unwrap();
    assert!(exec.ro.res.is_empty());
    assert!(exec.ro.nfa.insts.is_empty());
    assert!(exec.ro.dfa.insts.is_empty());
    assert!(exec.ro.dfa_reverse.insts.is_empty());
    assert_eq!(exec.ro.match_type, MatchType::Nothing);
}

#[test]
fn test_build_single_pattern() {
    let exec_builder = ExecBuilder::new("abc");
    let result = exec_builder.build();
    assert!(result.is_ok());
    let exec = result.unwrap();
    assert_eq!(exec.ro.res.len(), 1);
    assert_eq!(exec.ro.res[0], "abc");
}

#[test]
fn test_build_multiple_patterns() {
    let exec_builder = ExecBuilder::new_many(vec!["abc", "def"]);
    let result = exec_builder.build();
    assert!(result.is_ok());
    let exec = result.unwrap();
    assert_eq!(exec.ro.res.len(), 2);
    assert_eq!(exec.ro.res[0], "abc");
    assert_eq!(exec.ro.res[1], "def");
    assert!(exec.ro.nfa.insts.is_empty());
    assert!(exec.ro.dfa.insts.is_empty());
}

#[test]
fn test_build_with_unicode_option() {
    let mut options = RegexOptions::default();
    options.pats = vec!["éclair".to_owned()];
    options.unicode = true;
    let exec_builder = ExecBuilder::new_options(options.clone());
    let result = exec_builder.build();
    assert!(result.is_ok());
    let exec = result.unwrap();
    assert_eq!(exec.ro.res.len(), 1);
    assert_eq!(exec.ro.res[0], "éclair");
    assert!(exec.ro.dfa.only_utf8);
}

#[test]
fn test_build_with_empty_string_pattern() {
    let exec_builder = ExecBuilder::new("");
    let result = exec_builder.build();
    assert!(result.is_ok());
    let exec = result.unwrap();
    assert_eq!(exec.ro.res.len(), 1);
    assert_eq!(exec.ro.res[0], "");
}

