// Answer 0

#[test]
fn test_ages_valid_case() {
    use std::result;

    #[derive(Debug)]
    struct AgeIter {
        ages: &'static [(&'static str, &'static [(char, char)])],
    }
    
    #[derive(Debug)]
    pub enum Error {
        PropertyNotFound,
        PropertyValueNotFound,
    }
    
    type Result<T> = result::Result<T, Error>;

    const AGE_VERSIONS: &'static [(&'static str, &'static [(char, char)])] = &[
        // Sample data for testing.
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

    fn ages(canonical_age: &str) -> Result<AgeIter> {
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
            Some(i) => Ok(AgeIter { ages: &AGES[..i + 1] }),
        }
    }

    // Test with a valid age that should return Ok.
    let result = ages("V3_1");
    assert!(result.is_ok());
    if let Ok(age_iter) = result {
        assert_eq!(age_iter.ages.len(), 4); // V1_1, V2_0, V2_1, V3_0, V3_1
    }
}

#[test]
#[should_panic(expected = "ages are out of sync")]
fn test_ages_out_of_sync() {
    use std::result;

    #[derive(Debug)]
    struct AgeIter {
        ages: &'static [(&'static str, &'static [(char, char)])],
    }
    
    #[derive(Debug)]
    pub enum Error {
        PropertyNotFound,
        PropertyValueNotFound,
    }
    
    type Result<T> = result::Result<T, Error>;

    const AGE_VERSIONS: &'static [(&'static str, &'static [(char, char)])] = &[
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

    fn ages(canonical_age: &str) -> Result<AgeIter> {
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
        assert_eq!(AGES.len(), AGE_VERSIONS.len(), "ages are out of sync");

        let pos = AGES.iter().position(|&(age, _)| canonical_age == age);
        match pos {
            None => Err(Error::PropertyValueNotFound),
            Some(i) => Ok(AgeIter { ages: &AGES[..i + 1] }),
        }
    }

    // Invoke ages with a known value, but manipulate the ages constant to trigger a panic.
    let _ = ages("V2_0");
}

