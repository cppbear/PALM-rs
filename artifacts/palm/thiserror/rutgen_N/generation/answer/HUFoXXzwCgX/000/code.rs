// Answer 0

#[derive(Default)]
struct Request<'a> {
    data: &'a str,
}

struct Provider;

impl Provider {
    fn provide<'a>(&self, request: &mut Request<'a>) {
        request.data = "Provided Data";
    }
}

impl Provider {
    fn thiserror_provide<'a>(&'a self, request: &mut Request<'a>) {
        self.provide(request);
    }
}

#[test]
fn test_thiserror_provide() {
    let provider = Provider::default();
    let mut request = Request { data: "" };
    provider.thiserror_provide(&mut request);
    assert_eq!(request.data, "Provided Data");
}

#[test]
fn test_thiserror_provide_empty_request() {
    let provider = Provider::default();
    let mut request = Request { data: "" };
    provider.thiserror_provide(&mut request);
    assert_ne!(request.data, "");
}

