// Answer 0

#[derive(Debug)]
struct Lcg128CmDxsm64;

impl std::fmt::Display for Lcg128CmDxsm64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lcg128CmDxsm64 {{}}")
    }
}

#[test]
fn test_fmt() {
    let instance = Lcg128CmDxsm64;
    let result = format!("{}", instance);
    assert_eq!(result, "Lcg128CmDxsm64 {}");
}

