// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use http::request::{Request, Builder};
    use http::version::Version;

    #[test]
    fn test_set_http_version_to_http_2() {
        let req = Request::builder()
            .version(Version::HTTP_2)
            .body(())
            .unwrap();

        assert_eq!(req.version(), &Version::HTTP_2);
    }

    #[test]
    fn test_set_http_version_to_http_1_1() {
        let req = Request::builder()
            .version(Version::HTTP_1_1)
            .body(())
            .unwrap();

        assert_eq!(req.version(), &Version::HTTP_1_1);
    }

    #[test]
    #[should_panic]
    fn test_set_http_version_invalid() {
        // This test is designed to panic if the version is invalid;
        // In practice, you need an appropriate panic condition.
        let req = Request::builder()
            .version(unsafe { std::mem::transmute::<u8, Version>(999) }) // intentionally bad version
            .body(())
            .unwrap();
    }
}

