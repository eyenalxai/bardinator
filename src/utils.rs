use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::SystemTime;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn get_random_element<T>(array: &[T]) -> &T {
    let counter = COUNTER.fetch_add(5, Ordering::SeqCst);

    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as usize;

    let index = (seed + counter) % array.len();
    &array[index]
}
