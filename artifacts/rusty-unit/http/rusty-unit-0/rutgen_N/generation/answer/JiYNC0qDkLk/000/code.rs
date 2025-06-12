// Answer 0

#[derive(Debug)]
struct Status(u16);

impl Status {
    fn get(&self) -> u16 {
        self.0
    }

    fn is_client_error(&self) -> bool {
        (400..500).contains(&self.get())
    }
}

#[test]
fn test_is_client_error_within_range() {
    let status = Status(404);
    assert!(status.is_client_error());
}

#[test]
fn test_is_client_error_below_range() {
    let status = Status(399);
    assert!(!status.is_client_error());
}

#[test]
fn test_is_client_error_above_range() {
    let status = Status(500);
    assert!(!status.is_client_error());
}

#[test]
fn test_is_client_error_on_lower_bound() {
    let status = Status(400);
    assert!(status.is_client_error());
}

#[test]
fn test_is_client_error_on_upper_bound() {
    let status = Status(499);
    assert!(status.is_client_error());
}

