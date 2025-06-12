// Answer 0

#[derive(Debug)]
struct Status(u16);

impl Status {
    fn get(&self) -> u16 {
        self.0
    }
    
    pub fn is_informational(&self) -> bool {
        (100..200).contains(&self.get())
    }
}

#[test]
fn test_is_informational_within_range() {
    let status = Status(150);
    assert!(status.is_informational());
}

#[test]
fn test_is_informational_at_lower_bound() {
    let status = Status(100);
    assert!(status.is_informational());
}

#[test]
fn test_is_informational_at_upper_bound() {
    let status = Status(199);
    assert!(status.is_informational());
}

#[test]
fn test_is_informational_below_range() {
    let status = Status(99);
    assert!(!status.is_informational());
}

#[test]
fn test_is_informational_above_range() {
    let status = Status(200);
    assert!(!status.is_informational());
}

