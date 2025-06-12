// Answer 0

#[derive(Debug, PartialEq)]
enum Danger {
    Green,
    Yellow,
    Red,
}

struct Status {
    danger: Danger,
}

impl Status {
    fn set_yellow(&mut self) {
        if let Danger::Green = self.danger {
            self.danger = Danger::Yellow;
        }
    }
}

#[test]
fn test_set_yellow_when_danger_is_green() {
    let mut status = Status { danger: Danger::Green };
    status.set_yellow();
    assert_eq!(status.danger, Danger::Yellow);
}

#[test]
fn test_set_yellow_when_danger_is_yellow() {
    let mut status = Status { danger: Danger::Yellow };
    status.set_yellow();
    assert_eq!(status.danger, Danger::Yellow);
}

#[test]
fn test_set_yellow_when_danger_is_red() {
    let mut status = Status { danger: Danger::Red };
    status.set_yellow();
    assert_eq!(status.danger, Danger::Red);
}

