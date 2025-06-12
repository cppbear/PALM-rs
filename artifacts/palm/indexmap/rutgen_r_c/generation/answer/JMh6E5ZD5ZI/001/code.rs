// Answer 0

#[test]
fn test_key_ref_basic() {
    struct TestBucket {
        hash: HashValue,
        key: String,
        value: usize,
    }
    
    let bucket = Bucket {
        hash: HashValue(10),
        key: String::from("test_key"),
        value: 42,
    };

    let key_ref = bucket.key_ref();
    assert_eq!(key_ref, &String::from("test_key"));
}

#[test]
fn test_key_ref_empty_key() {
    struct TestBucket {
        hash: HashValue,
        key: String,
        value: usize,
    }
    
    let bucket = Bucket {
        hash: HashValue(20),
        key: String::new(),
        value: 0,
    };

    let key_ref = bucket.key_ref();
    assert_eq!(key_ref, &String::new());
}

#[test]
fn test_key_ref_special_character_key() {
    struct TestBucket {
        hash: HashValue,
        key: String,
        value: usize,
    }
    
    let bucket = Bucket {
        hash: HashValue(30),
        key: String::from("!@#$$%^&*()"),
        value: 100,
    };

    let key_ref = bucket.key_ref();
    assert_eq!(key_ref, &String::from("!@#$$%^&*()"));
}

#[test]
#[should_panic]
fn test_key_ref_panic_on_empty_key() {
    struct TestBucket {
        hash: HashValue,
        key: String,
        value: usize,
    }
    
    let bucket = Bucket {
        hash: HashValue(40),
        key: String::from(""),
        value: 1,
    };

    // Attempt to use an empty key should trigger a panic. 
    // In this case, we are only illustrating a panic scenario.
    let key_ref = bucket.key_ref();
    assert!(key_ref.is_empty());
}

