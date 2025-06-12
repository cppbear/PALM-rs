// Answer 0

#[test]
fn test_iter_basic() {
    struct TestRawIter;
    
    impl Clone for TestRawIter {
        fn clone(&self) -> Self {
            TestRawIter
        }
    }

    struct TestIterMut<'a> {
        inner: TestRawIter,
        marker: PhantomData<(&'a (), &'a mut ())>,
    }
    
    impl<'a> TestIterMut<'a> {
        fn iter(&self) -> Iter<'_, (), ()> {
            Iter {
                inner: self.inner.clone(),
                marker: PhantomData,
            }
        }
    }

    let iter_mut = TestIterMut {
        inner: TestRawIter,
        marker: PhantomData,
    };
    
    let iter = iter_mut.iter();
    assert!(iter.inner.iter == iter_mut.inner.clone());
}

#[test]
fn test_iter_empty() {
    struct TestRawIter;

    impl Clone for TestRawIter {
        fn clone(&self) -> Self {
            TestRawIter
        }
    }

    struct TestIterMut<'a> {
        inner: TestRawIter,
        marker: PhantomData<(&'a (), &'a mut ())>,
    }

    impl<'a> TestIterMut<'a> {
        fn iter(&self) -> Iter<'_, (), ()> {
            Iter {
                inner: self.inner.clone(),
                marker: PhantomData,
            }
        }
    }

    let iter_mut = TestIterMut {
        inner: TestRawIter,
        marker: PhantomData,
    };
    
    let iter = iter_mut.iter();
    assert!(iter.inner.iter == iter_mut.inner.clone());
}

#[test]
fn test_iter_clone() {
    struct TestRawIter;

    impl Clone for TestRawIter {
        fn clone(&self) -> Self {
            TestRawIter
        }
    }

    struct TestIterMut<'a> {
        inner: TestRawIter,
        marker: PhantomData<(&'a (), &'a mut ())>,
    }

    impl<'a> TestIterMut<'a> {
        fn iter(&self) -> Iter<'_, (), ()> {
            Iter {
                inner: self.inner.clone(),
                marker: PhantomData,
            }
        }
    }

    let iter_mut = TestIterMut {
        inner: TestRawIter,
        marker: PhantomData,
    };

    let iter1 = iter_mut.iter();
    let iter2 = iter_mut.iter();
    
    assert!(iter1.inner.iter == iter_mut.inner.clone());
    assert!(iter2.inner.iter == iter_mut.inner.clone());
}

