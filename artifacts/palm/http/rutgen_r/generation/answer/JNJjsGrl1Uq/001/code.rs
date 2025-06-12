// Answer 0

#[derive(Default)]
struct Response<T> {
    head: Head<T>,
}

#[derive(Default)]
struct Head<T> {
    version: Version,
}

#[derive(Debug, PartialEq)]
enum Version {
    HTTP_10,
    HTTP_11,
}

impl<T> Response<T> {
    pub fn version(&self) -> Version {
        self.head.version
    }
}

#[test]
fn test_version_default() {
    let response: Response<()> = Response::default();
    assert_eq!(response.version(), Version::HTTP_11);
}

#[test]
fn test_version_http_10() {
    let response = Response {
        head: Head {
            version: Version::HTTP_10,
        },
    };
    assert_eq!(response.version(), Version::HTTP_10);
}

#[test]
fn test_version_http_11() {
    let response = Response {
        head: Head {
            version: Version::HTTP_11,
        },
    };
    assert_eq!(response.version(), Version::HTTP_11);
}

