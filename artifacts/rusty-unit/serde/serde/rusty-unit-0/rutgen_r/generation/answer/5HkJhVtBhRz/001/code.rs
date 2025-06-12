// Answer 0

#[test]
fn test_expectation() {
    struct Expectation<'a> {
        expecting: &'a str,
    }

    impl<'a> Expectation<'a> {
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let mut output = String::new();
    {
        let expectation = Expectation { expecting: "expected value" };
        let result = expectation.expecting(&mut output).is_ok();
        assert!(result);
        assert_eq!(output, "expected value");
    }

    output.clear();
    {
        let expectation = Expectation { expecting: "" };
        let result = expectation.expecting(&mut output).is_ok();
        assert!(result);
        assert_eq!(output, "");
    }

    output.clear();
    {
        let expectation = Expectation { expecting: "another expected value" };
        let result = expectation.expecting(&mut output).is_ok();
        assert!(result);
        assert_eq!(output, "another expected value");
    }
}

