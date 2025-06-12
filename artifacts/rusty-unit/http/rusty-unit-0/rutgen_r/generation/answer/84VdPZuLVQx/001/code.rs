// Answer 0


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
fn test_is_redirection_with_status_300() {
    let status = Status(300);
    assert!(status.is_redirection());
}

#[test]
fn test_is_redirection_with_status_399() {
    let status = Status(399);
    assert!(status.is_redirection());
}

#[test]
fn test_is_redirection_with_status_299() {
    let status = Status(299);
    assert!(!status.is_redirection());
}

#[test]
fn test_is_redirection_with_status_400() {
    let status = Status(400);
    assert!(!status.is_redirection());
}

#[test]
fn test_is_redirection_with_boundary_case_301() {
    let status = Status(301);
    assert!(status.is_redirection());
}

#[test]
fn test_is_redirection_with_boundary_case_398() {
    let status = Status(398);
    assert!(status.is_redirection());
}

#[test]
fn test_is_redirection_with_negative_case_below_range() {
    let status = Status(100);
    assert!(!status.is_redirection());
}

#[test]
fn test_is_redirection_with_negative_case_above_range() {
    let status = Status(500);
    assert!(!status.is_redirection());
}

#[test]
#[should_panic]
fn test_is_redirection_with_invalid_status() {
    let status = Status(600);
    // This would not panic with the current implementation, but in a real scenario,
    // checking for panics may go here.
    assert!(!status.is_redirection());
}


