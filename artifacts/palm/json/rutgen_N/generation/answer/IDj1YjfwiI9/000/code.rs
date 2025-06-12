// Answer 0

#[derive(Debug)]
enum ParserNumber {
    F64(f64),
    U64(u64),
    I64(i64),
    #[cfg(feature = "arbitrary_precision")]
    String(String),
}

mod de {
    use super::*;

    #[derive(Debug)]
    pub enum Unexpected {
        Float(f64),
        Unsigned(u64),
        Signed(i64),
        #[cfg(feature = "arbitrary_precision")]
        Other(&'static str),
    }

    #[derive(Debug)]
    pub struct Error {
        unexpected: Unexpected,
        expected: &'static dyn Expected,
    }

    impl Error {
        pub fn invalid_type(unexpected: Unexpected, expected: &dyn Expected) -> Error {
            Error { unexpected, expected }
        }
    }
}

trait Expected {}

#[test]
fn test_invalid_type_f64() {
    let exp: &dyn Expected = &();
    let result = de::Error::invalid_type(de::Unexpected::Float(3.14), exp);
    assert_eq!(format!("{:?}", result.unexpected), "Float(3.14)");
}

#[test]
fn test_invalid_type_u64() {
    let exp: &dyn Expected = &();
    let result = de::Error::invalid_type(de::Unexpected::Unsigned(42), exp);
    assert_eq!(format!("{:?}", result.unexpected), "Unsigned(42)");
}

#[test]
fn test_invalid_type_i64() {
    let exp: &dyn Expected = &();
    let result = de::Error::invalid_type(de::Unexpected::Signed(-1), exp);
    assert_eq!(format!("{:?}", result.unexpected), "Signed(-1)");
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_invalid_type_string() {
    let exp: &dyn Expected = &();
    let result = de::Error::invalid_type(de::Unexpected::Other("number"), exp);
    assert_eq!(format!("{:?}", result.unexpected), "Other(\"number\")");
}

