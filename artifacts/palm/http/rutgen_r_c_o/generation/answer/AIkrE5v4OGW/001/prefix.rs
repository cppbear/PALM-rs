// Answer 0

#[test]
fn test_version_mut_initial_value() {
    let mut request: Request<()> = Request::new(());
    let version_mut_ref = request.version_mut();
}

#[test]
fn test_version_mut_update_to_http_1_1() {
    let mut request: Request<()> = Request::new(());
    *request.version_mut() = Version(1);
}

#[test]
fn test_version_mut_update_to_http_2() {
    let mut request: Request<()> = Request::new(());
    *request.version_mut() = Version(2);
}

#[test]
#[should_panic]
fn test_version_mut_exceeding_max_version() {
    let mut request: Request<()> = Request::new(());
    *request.version_mut() = Version(3);
}

#[test]
fn test_version_mut_multiple_updates() {
    let mut request: Request<()> = Request::new(());
    let version_mut_ref = request.version_mut();
    *version_mut_ref = Version(1);
    *version_mut_ref = Version(2);
}

