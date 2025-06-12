// Answer 0

#[test]
fn test_canonical_reason_for_100() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(100) });
    status.canonical_reason();
}

#[test]
fn test_canonical_reason_for_200() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(200) });
    status.canonical_reason();
}

#[test]
fn test_canonical_reason_for_404() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(404) });
    status.canonical_reason();
}

#[test]
fn test_canonical_reason_for_500() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(500) });
    status.canonical_reason();
}

#[test]
fn test_canonical_reason_for_511() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(511) });
    status.canonical_reason();
}

#[test]
fn test_canonical_reason_for_204() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(204) });
    status.canonical_reason();
}

#[test]
fn test_canonical_reason_for_301() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(301) });
    status.canonical_reason();
}

#[test]
fn test_canonical_reason_for_418() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(418) });
    status.canonical_reason();
}

#[test]
fn test_canonical_reason_for_405() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(405) });
    status.canonical_reason();
}

#[test]
fn test_canonical_reason_for_503() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(503) });
    status.canonical_reason();
}

