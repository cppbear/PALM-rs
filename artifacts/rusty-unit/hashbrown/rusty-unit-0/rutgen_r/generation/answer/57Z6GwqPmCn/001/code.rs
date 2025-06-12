// Answer 0

#[test]
fn test_write_valid_value() {
    struct TestStruct {
        value: i32,
    }

    let test_struct = TestStruct { value: 42 };
    let ptr = &test_struct as *const TestStruct as *mut TestStruct;

    unsafe {
        ptr.write(TestStruct { value: 100 });
        assert_eq!((*ptr).value, 100);
    }
}

#[test]
#[should_panic]
fn test_write_with_invalid_hash_eq() {
    // A hypothetical struct that does not implement Hash or Eq
    struct NonHashEqStruct {
        value: i32,
    }

    let non_hash_struct = NonHashEqStruct { value: 42 };
    let ptr = &non_hash_struct as *const NonHashEqStruct as *mut NonHashEqStruct;

    unsafe {
        ptr.write(NonHashEqStruct { value: 100 });
        // since we don't have Hash or Eq, this is expected to panic, but we won't reach this line
    }
}

#[test]
fn test_write_overwrites_memory() {
    struct TestStruct {
        value: i32,
    }

    let mut my_value = TestStruct { value: 5 };
    let ptr = &mut my_value as *mut TestStruct;

    unsafe {
        ptr.write(TestStruct { value: 10 });
        assert_eq!((*ptr).value, 10);
    }
}

#[test]
fn test_write_with_different_values() {
    struct MyStruct {
        number: i32,
    }

    let my_struct = MyStruct { number: 50 };
    let ptr = &my_struct as *const MyStruct as *mut MyStruct;

    unsafe {
        ptr.write(MyStruct { number: 200 });
        assert_eq!((*ptr).number, 200);
    }
}

