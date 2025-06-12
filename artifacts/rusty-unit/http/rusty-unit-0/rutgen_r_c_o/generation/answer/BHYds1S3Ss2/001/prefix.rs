// Answer 0

#[test]
fn test_is_informational_100() {
    let status_code = StatusCode(NonZeroU16::new(100).unwrap());
    status_code.is_informational();
}

#[test]
fn test_is_informational_199() {
    let status_code = StatusCode(NonZeroU16::new(199).unwrap());
    status_code.is_informational();
}

#[test]
fn test_is_informational_200() {
    let status_code = StatusCode(NonZeroU16::new(200).unwrap());
    status_code.is_informational();
}

#[test]
fn test_is_informational_150() {
    let status_code = StatusCode(NonZeroU16::new(150).unwrap());
    status_code.is_informational();
}

#[test]
fn test_is_informational_100_to_199() {
    for n in 100..200 {
        let status_code = StatusCode(NonZeroU16::new(n).unwrap());
        status_code.is_informational();
    }
}

