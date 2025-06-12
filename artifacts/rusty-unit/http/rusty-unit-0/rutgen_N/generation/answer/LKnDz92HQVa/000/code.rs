// Answer 0

#[derive(Debug)]
enum Danger {
    Green,
    Yellow,
    Red(i32),
}

impl Danger {
    fn is_red(&self) -> bool {
        matches!(*self, Danger::Red(_))
    }
}

#[test]
fn test_is_red_with_red() {
    let danger = Danger::Red(5);
    assert!(danger.is_red());
}

#[test]
fn test_is_red_with_green() {
    let danger = Danger::Green;
    assert!(!danger.is_red());
}

#[test]
fn test_is_red_with_yellow() {
    let danger = Danger::Yellow;
    assert!(!danger.is_red());
}

