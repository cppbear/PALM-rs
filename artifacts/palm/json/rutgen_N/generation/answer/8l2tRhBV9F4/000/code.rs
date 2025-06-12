// Answer 0

#[derive(Default)]
struct Delegate {
    failed: bool,
}

impl Delegate {
    fn set_failed(&mut self, failed: &mut bool) {
        self.failed = *failed;
    }
}

struct Example {
    delegate: Delegate,
}

impl Example {
    fn set_failed(&mut self, failed: &mut bool) {
        self.delegate.set_failed(failed);
    }
}

#[test]
fn test_set_failed() {
    let mut example = Example::default();
    let mut failed_flag = false;

    example.set_failed(&mut failed_flag);
    assert!(!example.delegate.failed);

    failed_flag = true;
    example.set_failed(&mut failed_flag);
    assert!(example.delegate.failed);
}

