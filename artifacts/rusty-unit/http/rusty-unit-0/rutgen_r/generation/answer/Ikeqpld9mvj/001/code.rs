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
fn test_is_server_error_within_bounds() {
    let status = Status(500);
    assert_eq!(status.is_server_error(), true);

    let status = Status(599);
    assert_eq!(status.is_server_error(), true);
}

#[test]
fn test_is_server_error_outside_bounds() {
    let status = Status(499);
    assert_eq!(status.is_server_error(), false);

    let status = Status(600);
    assert_eq!(status.is_server_error(), false);
}

#[test]
fn test_is_server_error_with_edge_cases() {
    let status = Status(500);
    assert!(status.is_server_error());

    let status = Status(599);
    assert!(status.is_server_error());

    let status = Status(501);
    assert!(status.is_server_error());

    let status = Status(598);
    assert!(status.is_server_error());
}

