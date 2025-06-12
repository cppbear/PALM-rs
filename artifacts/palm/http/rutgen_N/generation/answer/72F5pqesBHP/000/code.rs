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
            color: "Red".to_string(),
        };
    }
}

#[test]
fn test_set_red_when_color_is_yellow() {
    let mut danger = Danger {
        color: "Yellow".to_string(),
    };
    danger.set_red();
    assert_eq!(danger.color, "Red");
}

#[test]
#[should_panic]
fn test_set_red_when_color_is_not_yellow() {
    let mut danger = Danger {
        color: "Green".to_string(),
    };
    danger.set_red(); // This should panic due to the debug_assert
}

