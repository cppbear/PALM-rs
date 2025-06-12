// Answer 0

#[test]
fn test_advance_mut_no_panic() {
    let mut buf: Vec<u8> = Vec::with_capacity(10);
    let cnt = 10;

    unsafe {
        buf.advance_mut(cnt); // Should not panic as remaining == cnt
    }

    assert_eq!(buf.len(), cnt);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 0 but advancing by 1")]
fn test_advance_mut_panic() {
    let mut buf: Vec<u8> = Vec::with_capacity(0);
    let cnt = 1;

    unsafe {
        buf.advance_mut(cnt); // This will trigger panic since remaining < cnt
    }
}

#[test]
fn test_advance_mut_multiple_pains() {
    let mut buf: Vec<u8> = Vec::with_capacity(20);
    let cnt1 = 10;
    let cnt2 = 10;

    unsafe {
        buf.advance_mut(cnt1); // Should not panic
        buf.advance_mut(cnt2); // Should not panic
    }

    assert_eq!(buf.len(), cnt1 + cnt2);
}

#[test]
#[should_panic(expected = "advance out of bounds: the len is 10 but advancing by 15")]
fn test_advance_mut_exceed_capacity() {
    let mut buf: Vec<u8> = Vec::with_capacity(10);
    let cnt1 = 10;
    let cnt2 = 15;

    unsafe {
        buf.advance_mut(cnt1); // Should not panic
        buf.advance_mut(cnt2); // This will trigger panic since remaining < cnt
    }
}

