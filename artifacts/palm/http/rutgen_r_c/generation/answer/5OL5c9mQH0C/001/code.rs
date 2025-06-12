// Answer 0

#[test]
fn test_method_mut_initialization() {
    let mut request: Request<()> = Request::new(());
    let method_mut = request.method_mut();
    *method_mut = Method::default();
    assert_eq!(*request.method(), Method::default());
}

#[test]
fn test_method_mut_update() {
    let mut request: Request<()> = Request::new(());
    *request.method_mut() = Method(MethodInner::Put);
    assert_eq!(*request.method(), Method(MethodInner::Put));
}

#[test]
fn test_method_mut_multiple_updates() {
    let mut request: Request<()> = Request::new(());
    let method_mut = request.method_mut();
    *method_mut = Method(MethodInner::Get);
    assert_eq!(*request.method(), Method(MethodInner::Get));
    *method_mut = Method(MethodInner::Post);
    assert_eq!(*request.method(), Method(MethodInner::Post));
    *method_mut = Method(MethodInner::Delete);
    assert_eq!(*request.method(), Method(MethodInner::Delete));
}

