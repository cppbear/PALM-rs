// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use regex::Captures;

    #[test]
    fn test_captures_iter() {
        let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
        let text = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";

        let captures: Vec<Captures> = re.captures_iter(text).collect();

        assert_eq!(captures.len(), 3);
        assert_eq!(&captures[0]["title"], "Citizen Kane");
        assert_eq!(&captures[0]["year"], "1941");
        assert_eq!(&captures[1]["title"], "The Wizard of Oz");
        assert_eq!(&captures[1]["year"], "1939");
        assert_eq!(&captures[2]["title"], "M");
        assert_eq!(&captures[2]["year"], "1931");
    }

    #[test]
    fn test_captures_iter_no_matches() {
        let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
        let text = "No movie titles here.";

        let captures: Vec<Captures> = re.captures_iter(text).collect();

        assert_eq!(captures.len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_captures_iter_invalid_regex() {
        let re = Regex::new(r"'(?P<title>[^']+'\s+\((?P<year>\d{4})\)").unwrap(); // Missing a closing parenthesis
        let _ = re.captures_iter(""); // This line should panic
    }
}

