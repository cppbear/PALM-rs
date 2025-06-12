// Answer 0

#[derive(Debug)]
enum Danger {
    Yellow,
    Red,
    Green,
}

impl Danger {
    fn is_yellow(&self) -> bool {
        matches!(*self, Danger::Yellow)
    }
}

#[test]
fn test_is_yellow_returns_true_for_yellow() {
    let yellow_danger = Danger::Yellow;
    assert!(yellow_danger.is_yellow());
}

#[test]
fn test_is_yellow_returns_false_for_red() {
    let red_danger = Danger::Red;
    assert!(!red_danger.is_yellow());
}

#[test]
fn test_is_yellow_returns_false_for_green() {
    let green_danger = Danger::Green;
    assert!(!green_danger.is_yellow());
}

