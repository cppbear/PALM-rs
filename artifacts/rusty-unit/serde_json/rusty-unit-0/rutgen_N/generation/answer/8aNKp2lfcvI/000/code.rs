// Answer 0

#[derive(Default)]
struct TestR;

impl TestR {
    fn set_failed(_self: &mut TestR, _failed: &mut bool) {
        *_failed = true;
    }
}

#[test]
fn test_set_failed() {
    let mut r = TestR::default();
    let mut failed = false;

    r.set_failed(&mut failed);

    assert!(failed);
}

