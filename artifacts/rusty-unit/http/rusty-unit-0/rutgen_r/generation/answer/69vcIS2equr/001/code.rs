// Answer 0


struct Danger {
    color: Color,
}

impl Danger {
    fn is_yellow(&self) -> bool {
        matches!(self.color, Color::Yellow)
    }

    fn set_green(&mut self) {
        debug_assert!(self.is_yellow());
        self.color = Color::Green;
    }
}

#[derive(Debug, PartialEq)]
enum Color {
    Yellow,
    Green,
}

#[test]
fn test_set_green_success() {
    let mut danger = Danger {
        color: Color::Yellow,
    };
    danger.set_green();
    assert_eq!(danger.color, Color::Green);
}

#[test]
#[should_panic]
fn test_set_green_panic() {
    let mut danger = Danger {
        color: Color::Green,
    };
    danger.set_green();
}


