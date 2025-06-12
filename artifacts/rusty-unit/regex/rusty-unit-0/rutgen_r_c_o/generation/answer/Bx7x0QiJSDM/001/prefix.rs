// Answer 0

#[test]
fn test_map_quit_case() {
    let quit_result: Result<u32> = Result::Quit;
    let _ = quit_result.map(|x| x * 2);
}

#[test]
fn test_map_quit_case_empty() {
    let quit_result: Result<u32> = Result::Quit;
    let _ = quit_result.map(|_| 0);
}

#[test]
fn test_map_quit_case_no_operation() {
    let quit_result: Result<u32> = Result::Quit;
    let _ = quit_result.map(|x| x);
}

