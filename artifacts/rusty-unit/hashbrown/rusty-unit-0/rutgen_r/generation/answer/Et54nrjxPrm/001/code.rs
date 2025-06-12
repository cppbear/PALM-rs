// Answer 0

#[test]
fn test_fmt_empty() {
    #[derive(Clone)]
    struct Empty;

    impl std::fmt::Debug for Empty {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().finish()
        }
    }

    let empty = Empty;
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        empty.fmt(&mut formatter).unwrap();
    }
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_single_element() {
    #[derive(Clone)]
    struct SingleElement(i32);

    impl std::fmt::Debug for SingleElement {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entry(self.0).finish()
        }
    }

    let single = SingleElement(42);
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        single.fmt(&mut formatter).unwrap();
    }
    assert_eq!(output, "[42]");
}

#[test]
fn test_fmt_multiple_elements() {
    #[derive(Clone)]
    struct MultiElement(Vec<i32>);

    impl std::fmt::Debug for MultiElement {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.0.clone()).finish()
        }
    }

    let multi = MultiElement(vec![1, 2, 3]);
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        multi.fmt(&mut formatter).unwrap();
    }
    assert_eq!(output, "[1, 2, 3]");
}

#[test]
#[should_panic]
fn test_fmt_panic_on_clone() {
    #[derive(Clone)]
    struct PanickingClone;

    impl std::fmt::Debug for PanickingClone {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            panic!("This is a panic test.");
        }
    }

    let panicking = PanickingClone;
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new(&mut output);
        panicking.fmt(&mut formatter).unwrap();
    }
}

