// Answer 0

#[test]
fn test_case_fold_simple_lowercase() {
    struct TestStruct {
        set: CharacterSet,
    }

    let mut test_instance = TestStruct {
        set: CharacterSet::new().add_range('a', 'z'),
    };
    
    test_instance.case_fold_simple();
    
    assert!(test_instance.set.contains_range('a', 'z'));
    assert!(test_instance.set.contains_range('A', 'Z'));
}

#[test]
fn test_case_fold_simple_empty() {
    struct TestStruct {
        set: CharacterSet,
    }

    let mut test_instance = TestStruct {
        set: CharacterSet::new(),
    };

    test_instance.case_fold_simple();

    // An empty set should remain empty after case folding
    assert!(!test_instance.set.contains_range('A', 'Z'));
    assert!(!test_instance.set.contains_range('a', 'z'));
}

#[test]
fn test_case_fold_simple_mixed_case() {
    struct TestStruct {
        set: CharacterSet,
    }

    let mut test_instance = TestStruct {
        set: CharacterSet::new().add_range('G', 'L').add_range('n', 'p'),
    };

    test_instance.case_fold_simple();

    assert!(test_instance.set.contains_range('G', 'L'));
    assert!(test_instance.set.contains_range('g', 'l'));
    assert!(test_instance.set.contains_range('n', 'p'));
    assert!(test_instance.set.contains_range('N', 'P'));
}

#[test]
#[should_panic]
fn test_case_fold_simple_invalid_range() {
    struct TestStruct {
        set: CharacterSet,
    }

    let mut test_instance = TestStruct {
        set: CharacterSet::new().add_range('z', 'a'), // invalid range
    };

    test_instance.case_fold_simple(); // this should panic
}

