// Answer 0

#[test]
fn test_fmt_with_empty_inner() {
    use std::fmt;
    use std::marker::PhantomData;
    
    struct Iter<'a> {
        inner: std::slice::Iter<'a, i32>,
        marker: PhantomData<&'a ()>,
    }

    struct TestStruct {
        inner: Vec<i32>,
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(Iter {
                    inner: self.inner.iter(),
                    marker: PhantomData,
                })
                .finish()
        }
    }
    
    let test_struct = TestStruct { inner: vec![] };
    let output = format!("{:?}", test_struct);
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_with_single_element() {
    use std::fmt;
    use std::marker::PhantomData;

    struct Iter<'a> {
        inner: std::slice::Iter<'a, i32>,
        marker: PhantomData<&'a ()>,
    }

    struct TestStruct {
        inner: Vec<i32>,
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(Iter {
                    inner: self.inner.iter(),
                    marker: PhantomData,
                })
                .finish()
        }
    }

    let test_struct = TestStruct { inner: vec![1] };
    let output = format!("{:?}", test_struct);
    assert_eq!(output, "[1]");
}

#[test]
fn test_fmt_with_multiple_elements() {
    use std::fmt;
    use std::marker::PhantomData;

    struct Iter<'a> {
        inner: std::slice::Iter<'a, i32>,
        marker: PhantomData<&'a ()>,
    }

    struct TestStruct {
        inner: Vec<i32>,
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list()
                .entries(Iter {
                    inner: self.inner.iter(),
                    marker: PhantomData,
                })
                .finish()
        }
    }

    let test_struct = TestStruct { inner: vec![1, 2, 3] };
    let output = format!("{:?}", test_struct);
    assert_eq!(output, "[1, 2, 3]");
}

