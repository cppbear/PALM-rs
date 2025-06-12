pub fn new_many<I, S>(res: I) -> Self
            where S: AsRef<str>, I: IntoIterator<Item=S> {
        let mut opts = RegexOptions::default();
        opts.pats = res.into_iter().map(|s| s.as_ref().to_owned()).collect();
        Self::new_options(opts)
    }