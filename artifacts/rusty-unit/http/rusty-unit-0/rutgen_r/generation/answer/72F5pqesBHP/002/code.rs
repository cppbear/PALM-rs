// Answer 0

#[derive(Debug)]
struct Danger {
    color: String,
}

impl Danger {
    fn is_yellow(&self) -> bool {
        self.color == "Yellow"
    }

    fn set_red(&mut self) {
        debug_assert!(self.is_yellow());
        *self = Danger {
            color: String::from("Red"),
        };
    }
}

#[test]
#[should_panic]
fn test_set_red_when_not_yellow() {
    let mut danger = Danger { color: String::from("Red") }; // Initial color is not Yellow
    danger.set_red(); // This should trigger a panic because is_yellow() returns false
}

#[test]
fn test_set_red_when_yellow() {
    let mut danger = Danger { color: String::from("Yellow") }; // Initial color is Yellow
    danger.set_red(); // This should execute without panic
    assert_eq!(danger.color, "Red"); // Check if the color is set to Red
}

