// Answer 0

#[derive(Default)]
struct Response<T> {
    head: Head<T>,
}

#[derive(Default)]
struct Head<T> {
    version: Version,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Version {
    HTTP_1_0,
    HTTP_1_1,
    HTTP_2,
}

impl<T> Response<T> {
    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.head.version
    }

    pub fn version(&self) -> &Version {
        &self.head.version
    }
}

#[test]
fn test_version_mut_default() {
    let mut response: Response<()> = Response::default();
    *response.version_mut() = Version::HTTP_2;
    assert_eq!(response.version(), &Version::HTTP_2);
}

#[test]
fn test_version_mut_http_10() {
    let mut response: Response<()> = Response::default();
    *response.version_mut() = Version::HTTP_1_0;
    assert_eq!(response.version(), &Version::HTTP_1_0);
}

#[test]
fn test_version_mut_http_11() {
    let mut response: Response<()> = Response::default();
    *response.version_mut() = Version::HTTP_1_1;
    assert_eq!(response.version(), &Version::HTTP_1_1);
}

#[test]
fn test_version_mut_http_2() {
    let mut response: Response<()> = Response::default();
    *response.version_mut() = Version::HTTP_2;
    assert_eq!(response.version(), &Version::HTTP_2);
}

