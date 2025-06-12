// Answer 0

#[derive(Default)]
struct Response<T> {
    head: Head<T>,
}

#[derive(Default)]
struct Head<T> {
    version: Version,
}

#[derive(PartialEq, Debug)]
enum Version {
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
fn test_version_mut_initialization() {
    let mut response: Response<()> = Response::default();
    assert_eq!(response.version(), &Version::HTTP_1_0);
}

#[test]
fn test_version_mut_change() {
    let mut response: Response<()> = Response::default();
    *response.version_mut() = Version::HTTP_2;
    assert_eq!(response.version(), &Version::HTTP_2);
}

#[test]
fn test_version_mut_back_to_default() {
    let mut response: Response<()> = Response::default();
    *response.version_mut() = Version::HTTP_2;
    assert_eq!(response.version(), &Version::HTTP_2);
    *response.version_mut() = Version::HTTP_1_1;
    assert_eq!(response.version(), &Version::HTTP_1_1);
}

