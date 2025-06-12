// Answer 0

#[test]
fn test_is_match_quit() {
    let result = Result::<()>::Quit;
    let _ = result.is_match();
}

#[test]
fn test_is_match_no_match() {
    let result = Result::<()>::NoMatch(0);
    let _ = result.is_match();
}

#[test]
fn test_is_match_match() {
    let result = Result::<()>::Match(());
    let _ = result.is_match();
}

