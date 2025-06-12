// Answer 0

#[derive(Debug)]
struct Status(u16);

impl Status {
    pub fn get(&self) -> u16 {
        self.0
    }
    
    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.get())
    }
}

#[test]
fn test_is_server_error_within_range() {
    let status = Status(500);
    assert!(status.is_server_error());

    let status = Status(599);
    assert!(status.is_server_error());
}

#[test]
fn test_is_server_error_below_range() {
    let status = Status(499);
    assert!(!status.is_server_error());
}

#[test]
fn test_is_server_error_above_range() {
    let status = Status(600);
    assert!(!status.is_server_error());
}

