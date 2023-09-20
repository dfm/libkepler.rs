pub trait HouseholderModel<T> {
    fn evaluate<const ORDER: usize>(&self) -> T;
}

// At opt-level=2 or 3, this will be computed at compile time for any const input
const fn factorial(n: usize) -> usize {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

macro_rules! householder {
    ( $f0:expr, $a0:expr ) => {
        $f0 / $a0
    };
    ( $f0:expr, $a0:expr, $a1:expr ) => {{
        let f1 = $f0 - $d0 * $d1;
        f1 / $d0
    }};
}
