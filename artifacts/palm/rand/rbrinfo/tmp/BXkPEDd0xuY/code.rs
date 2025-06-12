fn sample_efraimidis_spirakis<R, F, X, N>(
    rng: &mut R,
    length: N,
    weight: F,
    amount: N,
) -> Result<IndexVec, WeightError>
where
    R: Rng + ?Sized,
    F: Fn(usize) -> X,
    X: Into<f64>,
    N: UInt,
    IndexVec: From<Vec<N>>,
{
    use std::{cmp::Ordering, collections::BinaryHeap};

    if amount == N::zero() {
        return Ok(IndexVec::U32(Vec::new()));
    }

    struct Element<N> {
        index: N,
        key: f64,
    }

    impl<N> PartialOrd for Element<N> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<N> Ord for Element<N> {
        fn cmp(&self, other: &Self) -> Ordering {
            // unwrap() should not panic since weights should not be NaN
            // We reverse so that BinaryHeap::peek shows the smallest item
            self.key.partial_cmp(&other.key).unwrap().reverse()
        }
    }

    impl<N> PartialEq for Element<N> {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    impl<N> Eq for Element<N> {}

    let mut candidates = BinaryHeap::with_capacity(amount.as_usize());
    let mut index = N::zero();
    while index < length && candidates.len() < amount.as_usize() {
        let weight = weight(index.as_usize()).into();
        if weight > 0.0 {
            // We use the log of the key used in A-ExpJ to improve precision
            // for small weights:
            let key = rng.random::<f64>().ln() / weight;
            candidates.push(Element { index, key });
        } else if !(weight >= 0.0) {
            return Err(WeightError::InvalidWeight);
        }

        index += N::one();
    }

    if index < length {
        let mut x = rng.random::<f64>().ln() / candidates.peek().unwrap().key;
        while index < length {
            let weight = weight(index.as_usize()).into();
            if weight > 0.0 {
                x -= weight;
                if x <= 0.0 {
                    let min_candidate = candidates.pop().unwrap();
                    let t = (min_candidate.key * weight).exp();
                    let key = rng.random_range(t..1.0).ln() / weight;
                    candidates.push(Element { index, key });

                    x = rng.random::<f64>().ln() / candidates.peek().unwrap().key;
                }
            } else if !(weight >= 0.0) {
                return Err(WeightError::InvalidWeight);
            }

            index += N::one();
        }
    }

    Ok(IndexVec::from(
        candidates.iter().map(|elt| elt.index).collect(),
    ))
}