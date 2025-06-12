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

struct Context {
    delegate: Delegate,
}

impl Context {
    fn set_failed(&mut self, failed: &mut bool) {
        self.delegate.set_failed(failed);
    }
}

#[test]
fn test_set_failed() {
    let mut context = Context::default();
    let mut failed = false;

    context.set_failed(&mut failed);
    assert_eq!(context.delegate.failed, false);

    failed = true;
    context.set_failed(&mut failed);
    assert_eq!(context.delegate.failed, true);
}

#[test]
#[should_panic]
fn test_set_failed_panic() {
    let mut context = Context::default();
    let mut failed = false;
    
    context.set_failed(&mut failed);
    failed = true;
    context.set_failed(&mut failed);
    panic!("This test is expected to panic");
}

