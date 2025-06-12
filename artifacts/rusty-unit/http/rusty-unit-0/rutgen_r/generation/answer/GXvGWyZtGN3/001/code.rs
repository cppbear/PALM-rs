// Answer 0

#[derive(Debug)]
struct Status(u16);

impl Status {
    fn get(&self) -> u16 {
        self.0
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.get())
    }
}

#[test]
fn test_is_success_with_success_status() {
    let status = Status(200);
    assert_eq!(status.is_success(), true);

    let status = Status(250);
    assert_eq!(status.is_success(), true);

    let status = Status(299);
    assert_eq!(status.is_success(), true);
}

#[test]
fn test_is_success_with_non_success_status() {
    let status = Status(199);
    assert_eq!(status.is_success(), false);

    let status = Status(300);
    assert_eq!(status.is_success(), false);

    let status = Status(1000);
    assert_eq!(status.is_success(), false);
}

