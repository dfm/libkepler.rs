pub use libkepler_householder_macro::kepler_householder_step;
use num_traits::MulAdd;

#[inline]
pub fn muladd<T: MulAdd + MulAdd<Output = T>>(x: T, a: T, b: T) -> T {
    x.mul_add(a, b)
}

#[macro_export]
macro_rules! horner {
    ( $x:expr, $a0:expr ) => { $a0 };
    ( $x:expr, $a0:expr, $a1:expr ) => {
        libkepler_householder::muladd($a1, $x, $a0)
    };
    ( $x:expr, $a0:expr, $( $a1:expr ),+ ) => {
        libkepler_householder::muladd( libkepler_householder::horner!( $x, $( $a1 ),+ ), $x, $a0 )
    };
}
