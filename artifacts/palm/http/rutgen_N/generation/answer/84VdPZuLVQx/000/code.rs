// Answer 0

#[derive(Debug)]
struct Status(u16);

impl Status {
    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn is_redirection(&self) -> bool {
        (300..400).contains(&self.get())
    }
}

#[test]
fn test_is_redirection_with_lower_bound() {
    let status = Status(300);
    assert_eq!(status.is_redirection(), true);
}

#[test]
fn test_is_redirection_with_middle_value() {
    let status = Status(350);
    assert_eq!(status.is_redirection(), true);
}

#[test]
fn test_is_redirection_with_upper_bound() {
    let status = Status(399);
    assert_eq!(status.is_redirection(), true);
}

#[test]
fn test_is_redirection_below_lower_bound() {
    let status = Status(299);
    assert_eq!(status.is_redirection(), false);
}

#[test]
fn test_is_redirection_above_upper_bound() {
    let status = Status(400);
    assert_eq!(status.is_redirection(), false);
}

