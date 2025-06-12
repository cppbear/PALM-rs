// Answer 0

#[derive(Debug)]
enum Danger {
    Red(i32),
    Yellow,
    Green,
}

impl Danger {
    fn is_red(&self) -> bool {
        matches!(*self, Danger::Red(_))
    }
}

#[test]
fn test_is_red_false_with_yellow() {
    let danger = Danger::Yellow;
    assert_eq!(danger.is_red(), false);
}

#[test]
fn test_is_red_false_with_green() {
    let danger = Danger::Green;
    assert_eq!(danger.is_red(), false);
}

