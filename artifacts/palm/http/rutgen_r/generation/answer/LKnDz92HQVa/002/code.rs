// Answer 0

#[derive(Debug)]
enum Danger {
    Red(i32), // Using i32 as a representative type for the inner value
    Yellow,
    Green,
}

impl Danger {
    fn is_red(&self) -> bool {
        matches!(*self, Danger::Red(_))
    }
}

#[test]
fn test_is_red_with_red_variant() {
    let danger = Danger::Red(5); // Any integer value will satisfy the condition
    assert_eq!(danger.is_red(), true);
}

#[test]
fn test_is_red_with_yellow_variant() {
    let danger = Danger::Yellow;
    assert_eq!(danger.is_red(), false);
}

#[test]
fn test_is_red_with_green_variant() {
    let danger = Danger::Green;
    assert_eq!(danger.is_red(), false);
}

