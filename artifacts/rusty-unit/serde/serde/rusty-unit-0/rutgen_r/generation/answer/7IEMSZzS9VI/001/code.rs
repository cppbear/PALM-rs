// Answer 0


struct ExpectingTest {
    expecting: &'static str,
}

impl ExpectingTest {
    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(self.expecting)
    }
}

#[test]
fn test_expecting() {
    let test_case = ExpectingTest {
        expecting: "Test string",
    };
    
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        assert!(test_case.expecting(&mut formatter).is_ok());
    }

    assert_eq!(output, "Test string");
}

#[test]
fn test_expecting_empty() {
    let test_case = ExpectingTest {
        expecting: "",
    };
    
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        assert!(test_case.expecting(&mut formatter).is_ok());
    }

    assert_eq!(output, "");
}

#[should_panic]
#[test]
fn test_expecting_panic() {
    let test_case = ExpectingTest {
        expecting: "This will panic",
    };

    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        // Assuming this may potentially cause a panic situation, adjust it as necessary
        // For this case, we simulate a situation where fmt.write_str might panic
        let _ = std::panic::catch_unwind(|| {
            test_case.expecting(&mut formatter).unwrap();
        });
    }
}


