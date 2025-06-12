// Answer 0

#[test]
fn test_ages_valid_input_v1_1() {
    struct AgeIter<'a> {
        ages: &'a [(&'static str, &'static [(char, char)])],
    }

    #[derive(Debug)]
    enum Error {
        PropertyValueNotFound,
    }
    
    fn ages(canonical_age: &str) -> Result<AgeIter, Error> {
        const AGES: &'static [(&'static str, &'static [(char, char)])] = &[
            ("V1_1", &[]),
            // other age definitions...
        ];

        let pos = AGES.iter().position(|&(age, _)| canonical_age == age);
        match pos {
            None => Err(Error::PropertyValueNotFound),
            Some(i) => Ok(AgeIter { ages: &AGES[..i+1] }),
        }
    }

    let result = ages("V1_1");
    assert!(result.is_ok());
}

#[test]
fn test_ages_valid_input_v2_0() {
    struct AgeIter<'a> {
        ages: &'a [(&'static str, &'static [(char, char)])],
    }

    #[derive(Debug)]
    enum Error {
        PropertyValueNotFound,
    }
    
    fn ages(canonical_age: &str) -> Result<AgeIter, Error> {
        const AGES: &'static [(&'static str, &'static [(char, char)])] = &[
            ("V1_1", &[]),
            ("V2_0", &[]),
            // other age definitions...
        ];

        let pos = AGES.iter().position(|&(age, _)| canonical_age == age);
        match pos {
            None => Err(Error::PropertyValueNotFound),
            Some(i) => Ok(AgeIter { ages: &AGES[..i+1] }),
        }
    }

    let result = ages("V2_0");
    assert!(result.is_ok());
}

#[test]
fn test_ages_valid_input_v10_0() {
    struct AgeIter<'a> {
        ages: &'a [(&'static str, &'static [(char, char)])],
    }

    #[derive(Debug)]
    enum Error {
        PropertyValueNotFound,
    }
    
    fn ages(canonical_age: &str) -> Result<AgeIter, Error> {
        const AGES: &'static [(&'static str, &'static [(char, char)])] = &[
            ("V10_0", &[]),
            // other age definitions...
        ];

        let pos = AGES.iter().position(|&(age, _)| canonical_age == age);
        match pos {
            None => Err(Error::PropertyValueNotFound),
            Some(i) => Ok(AgeIter { ages: &AGES[..i+1] }),
        }
    }

    let result = ages("V10_0");
    assert!(result.is_ok());
}

#[test]
fn test_ages_invalid_input() {
    struct AgeIter<'a> {
        ages: &'a [(&'static str, &'static [(char, char)])],
    }

    #[derive(Debug)]
    enum Error {
        PropertyValueNotFound,
    }
    
    fn ages(canonical_age: &str) -> Result<AgeIter, Error> {
        const AGES: &'static [(&'static str, &'static [(char, char)])] = &[
            ("V1_1", &[]),
            // other age definitions...
        ];

        let pos = AGES.iter().position(|&(age, _)| canonical_age == age);
        match pos {
            None => Err(Error::PropertyValueNotFound),
            Some(i) => Ok(AgeIter { ages: &AGES[..i+1] }),
        }
    }

    let result = ages("Invalid_Age");
    assert!(result.is_err());
}

