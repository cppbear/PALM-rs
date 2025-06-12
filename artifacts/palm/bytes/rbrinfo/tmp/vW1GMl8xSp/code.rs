unsafe fn shared_v_is_unique(data: &AtomicPtr<()>) -> bool {
    let shared = data.load(Ordering::Acquire);
    let ref_count = (*shared.cast::<Shared>()).ref_count.load(Ordering::Relaxed);
    ref_count == 1
}