// Answer 0

#[test]
fn test_ages_invalid_canonical_age() {
    struct DummyError;
    struct AgeIter<'a> {
        ages: &'a [(&'static str, &'static [(char, char)])],
    }

    impl AgeIter<'_> {
        // Dummy implementation for the sake of the test
    }

    enum Error {
        PropertyValueNotFound,
    }

    fn ages(canonical_age: &str) -> Result<AgeIter, Error> {
        const AGES: &'static [(&'static str, &'static [(char, char)])] = &[
            ("V1_1", &[]), // Dummy age definitions for the test
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

    let result = ages("V99_0");
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

#[test]
fn test_ages_edge_case_empty_string() {
    struct DummyError;
    struct AgeIter<'a> {
        ages: &'a [(&'static str, &'static [(char, char)])],
    }

    impl AgeIter<'_> {
        // Dummy implementation for the sake of the test
    }

    enum Error {
        PropertyValueNotFound,
    }

    fn ages(canonical_age: &str) -> Result<AgeIter, Error> {
        const AGES: &'static [(&'static str, &'static [(char, char)])] = &[
            ("V1_1", &[]), // Dummy age definitions for the test
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

    let result = ages("");
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

