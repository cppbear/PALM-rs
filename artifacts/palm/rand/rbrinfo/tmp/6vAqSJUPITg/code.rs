pub fn fill<T: Fill + ?Sized>(dest: &mut T) {
    dest.fill(&mut rng())
}