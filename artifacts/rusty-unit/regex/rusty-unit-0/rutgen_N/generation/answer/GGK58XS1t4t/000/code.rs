// Answer 0

#[test]
fn test_literals_empty() {
    struct TestStruct {
        lits: Vec<Literal>,
    }

    impl TestStruct {
        pub fn literals(&self) -> &[Literal] {
            &self.lits
        }
    }

    let test_instance = TestStruct { lits: Vec::new() };
    assert_eq!(test_instance.literals().len(), 0);
}

#[test]
fn test_literals_non_empty() {
    struct TestStruct {
        lits: Vec<Literal>,
    }

    impl TestStruct {
        pub fn literals(&self) -> &[Literal] {
            &self.lits
        }
    }

    let test_instance = TestStruct { lits: vec![Literal::from('a'), Literal::from('b')] };
    let literals = test_instance.literals();
    assert_eq!(literals.len(), 2);
    assert_eq!(literals[0], Literal::from('a'));
    assert_eq!(literals[1], Literal::from('b'));
}

#[test]
fn test_literals_order_unspecified() {
    struct TestStruct {
        lits: Vec<Literal>,
    }

    impl TestStruct {
        pub fn literals(&self) -> &[Literal] {
            &self.lits
        }
    }

    let mut test_instance = TestStruct { lits: vec![Literal::from('a'), Literal::from('b'), Literal::from('c')] };
    let literals = test_instance.literals();
    assert!(literals.contains(&Literal::from('a')));
    assert!(literals.contains(&Literal::from('b')));
    assert!(literals.contains(&Literal::from('c')));
}

