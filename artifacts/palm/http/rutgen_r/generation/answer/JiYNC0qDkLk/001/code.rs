// Answer 0


struct Status(u16);

impl Status {
    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.get())
    }
}

#[test]
fn test_is_client_error_in_range() {
    let status = Status(404);
    assert!(status.is_client_error());
}

#[test]
fn test_is_client_error_lower_bound() {
    let status = Status(400);
    assert!(status.is_client_error());
}

#[test]
fn test_is_client_error_upper_bound() {
    let status = Status(499);
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
fn test_is_client_error_at_negative_bound() {
    let status = Status(0);
    assert!(!status.is_client_error());
}


