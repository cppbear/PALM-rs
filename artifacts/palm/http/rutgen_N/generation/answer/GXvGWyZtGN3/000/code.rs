// Answer 0

#[derive(Debug)]
struct Status(u16);

impl Status {
    pub fn new(code: u16) -> Self {
        Status(code)
    }

    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.get())
    }
}

#[test]
fn test_is_success_with_success_status() {
    let status = Status::new(200);
    assert!(status.is_success());
}

#[test]
fn test_is_success_with_success_status_mid() {
    let status = Status::new(250);
    assert!(status.is_success());
}

#[test]
fn test_is_success_with_success_status_upper_bound() {
    let status = Status::new(299);
    assert!(status.is_success());
}

#[test]
fn test_is_success_with_failure_status_lower_bound() {
    let status = Status::new(199);
    assert!(!status.is_success());
}

#[test]
fn test_is_success_with_failure_status_above_upper_bound() {
    let status = Status::new(300);
    assert!(!status.is_success());
}

