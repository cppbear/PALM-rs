// Answer 0


// src/status.rs

struct Status(u16);

impl Status {
    pub fn get(&self) -> u16 {
        self.0
    }

    /// Check if status is within 100-199.
    pub fn is_informational(&self) -> bool {
        (100..200).contains(&self.0.get())
    }
}

#[test]
fn test_informational_status_lower_bound() {
    let status = Status(100);
    assert!(status.is_informational());
}

#[test]
fn test_informational_status_upper_bound() {
    let status = Status(199);
    assert!(status.is_informational());
}

#[test]
fn test_informational_status_below_lower_bound() {
    let status = Status(99);
    assert!(!status.is_informational());
}

#[test]
fn test_informational_status_above_upper_bound() {
    let status = Status(200);
    assert!(!status.is_informational());
}

#[test]
fn test_informational_status_exactly_boundary() {
    let status = Status(150);
    assert!(status.is_informational());
}


