// Answer 0

#[derive(Default)]
struct Head {
    uri: Uri,
}

#[derive(Default)]
struct Uri(&'static str);

struct Request<T> {
    head: Head,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for Request<T> {
    fn default() -> Self {
        Self {
            head: Head {
                uri: Uri("/"),
            },
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Request<T> {
    pub fn uri(&self) -> &Uri {
        &self.head.uri
    }
}

#[test]
fn test_uri_default() {
    let request: Request<()> = Request::default();
    assert_eq!(*request.uri(), *Uri("/"));
}

#[test]
fn test_uri_non_default() {
    struct CustomHead {
        uri: Uri,
    }

    let custom_request = Request {
        head: Head {
            uri: Uri("/custom"),
        },
        _marker: std::marker::PhantomData,
    };
    assert_eq!(*custom_request.uri(), *Uri("/custom"));
}

