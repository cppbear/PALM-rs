// Answer 0

#[test]
fn test_is_success_for_200() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(200) });
    status.is_success();
}

#[test]
fn test_is_success_for_250() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(250) });
    status.is_success();
}

#[test]
fn test_is_success_for_299() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(299) });
    status.is_success();
}

#[test]
fn test_is_success_for_201() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(201) });
    status.is_success();
}

#[test]
fn test_is_success_for_207() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(207) });
    status.is_success();
}

#[test]
fn test_is_success_for_204() {
    let status = StatusCode(unsafe { NonZeroU16::new_unchecked(204) });
    status.is_success();
}

