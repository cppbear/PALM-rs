// Answer 0

#[test]
fn test_ages_valid_version() {
    struct AgeIter<'a> {
        ages: &'a [(&'static str, &'static [(char, char)])],
    }

    struct Error;
    impl Error {
        const PropertyValueNotFound: Error = Error;
    }

    fn ages(canonical_age: &str) -> Result<AgeIter, Error> {
        const AGES: &'static [(&'static str, &'static [(char, char)])] = &[
            ("V1_1", &[]),
            ("V2_0", &[]),
            ("V2_1", &[]),
            ("V3_0", &[]),
            ("V3_1", &[]),
            ("V3_2", &[]),
            ("V4_0", &[]),
            ("V4_1", &[]),
            ("V5_0", &[]),
            ("V5_1", &[]),
            ("V5_2", &[]),
            ("V6_0", &[]),
            ("V6_1", &[]),
            ("V6_2", &[]),
            ("V6_3", &[]),
            ("V7_0", &[]),
            ("V8_0", &[]),
            ("V9_0", &[]),
            ("V10_0", &[]),
        ];
        let pos = AGES.iter().position(|&(age, _)| canonical_age == age);
        match pos {
            None => Err(Error::PropertyValueNotFound),
            Some(i) => Ok(AgeIter { ages: &AGES[..i+1] }),
        }
    }

    let result = ages("V3_0");
    assert!(result.is_ok());
}

#[test]
fn test_ages_invalid_version() {
    struct AgeIter<'a> {
        ages: &'a [(&'static str, &'static [(char, char)])],
    }

    struct Error;
    impl Error {
        const PropertyValueNotFound: Error = Error;
    }

    fn ages(canonical_age: &str) -> Result<AgeIter, Error> {
        const AGES: &'static [(&'static str, &'static [(char, char)])] = &[
            ("V1_1", &[]),
            ("V2_0", &[]),
            ("V2_1", &[]),
            ("V3_0", &[]),
            ("V3_1", &[]),
            ("V3_2", &[]),
            ("V4_0", &[]),
            ("V4_1", &[]),
            ("V5_0", &[]),
            ("V5_1", &[]),
            ("V5_2", &[]),
            ("V6_0", &[]),
            ("V6_1", &[]),
            ("V6_2", &[]),
            ("V6_3", &[]),
            ("V7_0", &[]),
            ("V8_0", &[]),
            ("V9_0", &[]),
            ("V10_0", &[]),
        ];
        let pos = AGES.iter().position(|&(age, _)| canonical_age == age);
        match pos {
            None => Err(Error::PropertyValueNotFound),
            Some(i) => Ok(AgeIter { ages: &AGES[..i+1] }),
        }
    }

    let result = ages("Invalid_Version");
    assert!(result.is_err());
    if let Err(Error::PropertyValueNotFound) = result {
        // Expected error
    } else {
        panic!("Expected PropertyValueNotFound error");
    }
}

