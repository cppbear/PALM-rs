// Answer 0

#[test]
fn test_fmt_with_valid_map() {
    use std::fmt;

    struct MyMap {
        map: Vec<&'static str>,
    }

    impl MyMap {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            for item in &self.map {
                write!(formatter, "{} ", item)?;
            }
            Ok(())
        }
    }

    let my_map = MyMap {
        map: vec!["item1", "item2", "item3"],
    };
    
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| my_map.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output.trim(), "item1 item2 item3");
}

#[test]
fn test_fmt_with_empty_map() {
    use std::fmt;

    struct MyMap {
        map: Vec<&'static str>,
    }

    impl MyMap {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            for item in &self.map {
                write!(formatter, "{} ", item)?;
            }
            Ok(())
        }
    }

    let my_map = MyMap {
        map: vec![],
    };
    
    let mut output = String::new();
    let result = fmt::write(&mut output, |f| my_map.fmt(f));
    
    assert!(result.is_ok());
    assert_eq!(output.trim(), "");
}

#[test]
#[should_panic]
fn test_fmt_with_panic() {
    use std::fmt;

    struct MyMap {
        map: Vec<Option<&'static str>>,
    }

    impl MyMap {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            for item in &self.map {
                // This will panic if the item is None
                write!(formatter, "{} ", item.unwrap())?;
            }
            Ok(())
        }
    }

    let my_map = MyMap {
        map: vec![Some("item1"), None], // triggers panic
    };
    
    let _ = fmt::write(&mut String::new(), |f| my_map.fmt(f));
}

