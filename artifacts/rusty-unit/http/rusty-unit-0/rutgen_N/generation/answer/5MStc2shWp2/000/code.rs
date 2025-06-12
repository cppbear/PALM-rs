// Answer 0

#[derive(Debug)]
struct Request {
    inner: Result<RequestInner, ()>,
}

#[derive(Debug)]
struct RequestInner {
    method: Method,
}

#[derive(Debug, PartialEq)]
enum Method {
    GET,
    POST,
}

impl Request {
    pub fn builder() -> Self {
        Request {
            inner: Ok(RequestInner { method: Method::GET }),
        }
    }

    pub fn method(mut self, method: &str) -> Self {
        self.inner = match method {
            "POST" => Ok(RequestInner { method: Method::POST }),
            _ => Ok(RequestInner { method: Method::GET }),
        };
        self
    }

    pub fn method_ref(&self) -> Option<&Method> {
        self.inner.as_ref().ok().map(|h| &h.method)
    }
}

#[test]
fn test_method_ref_default() {
    let req = Request::builder();
    assert_eq!(req.method_ref(), Some(&Method::GET));
}

#[test]
fn test_method_ref_post() {
    let req = Request::builder().method("POST");
    assert_eq!(req.method_ref(), Some(&Method::POST));
}

#[test]
fn test_method_ref_invalid() {
    let req = Request::builder().method("INVALID");
    assert_eq!(req.method_ref(), Some(&Method::GET));
}

