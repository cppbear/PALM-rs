// Answer 0

#[derive(Debug)]
enum Danger {
    Red,
    Yellow,
    Green,
}

impl Danger {
    fn is_yellow(&self) -> bool {
        matches!(*self, Danger::Yellow)
    }
}

#[test]
fn test_is_yellow_true() {
    let yellow_danger = Danger::Yellow;
    assert!(yellow_danger.is_yellow());
}

#[test]
fn test_is_yellow_false_red() {
    let red_danger = Danger::Red;
    assert!(!red_danger.is_yellow());
}

#[test]
fn test_is_yellow_false_green() {
    let green_danger = Danger::Green;
    assert!(!green_danger.is_yellow());
}

