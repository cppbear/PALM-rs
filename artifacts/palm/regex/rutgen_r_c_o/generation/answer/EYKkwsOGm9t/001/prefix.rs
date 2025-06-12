// Answer 0

#[test]
fn test_matches_exact_start() {
    let instruction = InstBytes { goto: 0, start: 100, end: 200 };
    instruction.matches(100);
}

#[test]
fn test_matches_middle() {
    let instruction = InstBytes { goto: 0, start: 100, end: 200 };
    instruction.matches(150);
}

#[test]
fn test_matches_exact_end() {
    let instruction = InstBytes { goto: 0, start: 100, end: 200 };
    instruction.matches(200);
}

#[test]
fn test_does_not_match_below_start() {
    let instruction = InstBytes { goto: 0, start: 100, end: 200 };
    instruction.matches(99);
}

#[test]
fn test_does_not_match_above_end() {
    let instruction = InstBytes { goto: 0, start: 100, end: 200 };
    instruction.matches(201);
}

