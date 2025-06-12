// Answer 0

#[test]
fn test_fmt_vacant_entry() {
    struct VacantEntry {
        value: Option<i32>,
    }

    impl VacantEntry {
        fn get(&self) -> &Option<i32> {
            &self.value
        }
        
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("VacantEntry").field(self.get()).finish()
        }
    }

    let entry1 = VacantEntry { value: None };
    let entry2 = VacantEntry { value: Some(42) };

    let mut output1 = String::new();
    let mut output2 = String::new();

    {
        let mut formatter = std::fmt::Formatter::new(&mut output1);
        entry1.fmt(&mut formatter).unwrap();
    }

    {
        let mut formatter = std::fmt::Formatter::new(&mut output2);
        entry2.fmt(&mut formatter).unwrap();
    }

    assert_eq!(output1, "VacantEntry(None)");
    assert_eq!(output2, "VacantEntry(Some(42))");
}

#[test]
#[should_panic]
fn test_fmt_vacant_entry_panic() {
    struct VacantEntry {
        value: Option<i32>,
    }

    impl VacantEntry {
        fn get(&self) -> &Option<i32> {
            panic!("Intentional panic for testing");
        }
        
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("VacantEntry").field(self.get()).finish()
        }
    }

    let entry = VacantEntry { value: None };
    let _ = entry.fmt(&mut std::fmt::Formatter::new(&mut String::new()));
}

