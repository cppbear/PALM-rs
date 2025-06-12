// Answer 0

#[test]
fn test_fmt() {
    struct Bucket<K> {
        key: K,
    }

    impl<K> Bucket<K> {
        fn key_ref(&self) -> &K {
            &self.key
        }
    }

    struct MyIterator<K> {
        iter: Vec<Bucket<K>>,
    }

    impl<K> MyIterator<K> {
        fn as_slice(&self) -> &[Bucket<K>] {
            &self.iter
        }
    }

    struct Formatter;

    impl fmt::Write for Formatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    impl fmt::Debug for MyIterator<String> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let iter = self.as_slice().iter().map(Bucket::key_ref);
            f.debug_list().entries(iter).finish()
        }
    }

    let buckets = vec![Bucket { key: "one".to_string() }, Bucket { key: "two".to_string() }];
    let my_iter = MyIterator { iter: buckets };
    
    let mut formatter = Formatter;
    let result = fmt::Debug::fmt(&my_iter, &mut formatter);
    
    assert!(result.is_ok());
}

