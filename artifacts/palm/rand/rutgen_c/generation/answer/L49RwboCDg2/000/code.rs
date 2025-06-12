// Answer 0

#[test]
fn test_thread_rng_debug_fmt() {
    struct DummyCore;
    struct DummyOsRng;

    impl rand_core::RngCore for DummyOsRng {
        fn next_u32(&mut self) -> u32 {
            0
        }

        fn next_u64(&mut self) -> u64 {
            0
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(0);
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    impl rand_core::CryptoRng for DummyOsRng {}

    impl<R, Rsdr> fmt::Debug for ReseedingRng<R, Rsdr>
    where
        R: rand_core::RngCore,
        Rsdr: rand_core::TryRngCore,
    {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            write!(_, "ReseedingRng {{ .. }}")
        }
    }

    let rng = ReseedingRng::<DummyCore, DummyOsRng>(DummyCore);
    let thread_rng = ThreadRng {
        rng: Rc::new(UnsafeCell::new(rng)),
    };

    let mut output = String::new();
    {
        let formatter = &mut fmt::Formatter::new(&mut output);
        thread_rng.fmt(formatter).unwrap();
    }
    assert_eq!(output, "ThreadRng {{ .. }}");
}

