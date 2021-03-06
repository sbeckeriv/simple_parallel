extern crate simple_parallel;

use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

// with this many elements, its (hopefully) likely that the
// threads won't execute sequentially.
const N: usize = 10000;

#[test]
fn probabilistic_out_of_ordering() {
    let mut index = (0..N).map(|_| 0).collect::<Vec<_>>();

    static ORDER: AtomicUsize = ATOMIC_USIZE_INIT;
    ORDER.store(0, Ordering::SeqCst);

    simple_parallel::for_(&mut index, |x| {
        *x = ORDER.fetch_add(1, Ordering::SeqCst);
    });

    assert!(index.iter().zip(index[1..].iter()).any(|(a, b)| a > b));
}

#[test]
fn pool() {
    let mut pool = simple_parallel::Pool::new(8);

    let mut index = (0..N).map(|_| 0usize).collect::<Vec<_>>();

    static ORDER: AtomicUsize = ATOMIC_USIZE_INIT;
    ORDER.store(0, Ordering::SeqCst);

    pool.for_(&mut index, |x| {
        *x = ORDER.fetch_add(1, Ordering::SeqCst);
    });

    assert!(index.iter().zip(index[1..].iter()).any(|(a, b)| a > b));
}
