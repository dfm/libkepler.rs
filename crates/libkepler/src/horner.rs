// Ref: https://github.com/andrewjradcliffe/horner-eval
use num_traits::MulAdd;

#[inline]
pub fn muladd<T: MulAdd + MulAdd<Output = T>>(x: T, a: T, b: T) -> T {
    x.mul_add(a, b)
}

#[macro_export]
macro_rules! horner {
    ( $x:expr, $a0:expr ) => { $a0 };
    ( $x:expr, $a0:expr, $a1:expr ) => {
        muladd($a1, $x, $a0)
    };
    ( $x:expr, $a0:expr, $( $a1:expr ),+ ) => {
        muladd( horner!( $x, $( $a1 ),+ ), $x, $a0 )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    macro_rules! test_horner_poly_eval {
        ($t:tt) => {
            let x: $t = 2 as $t;
            let a0: $t = 1 as $t;
            let a1: $t = 2 as $t;
            let a2: $t = 3 as $t;
            let expected = a0 + a1 * x + a2 * x * x;
            let actual = horner!(x, a0, a1, a2);
            assert_abs_diff_eq!(actual, expected);
        };
    }

    #[test]
    fn test_horner_poly_eval_i32() {
        test_horner_poly_eval!(i32);
    }

    #[test]
    fn test_horner_poly_eval_i64() {
        test_horner_poly_eval!(i64);
    }

    #[test]
    fn test_horner_poly_eval_f64() {
        test_horner_poly_eval!(f64);
    }

    #[test]
    fn test_horner_poly_eval_f32() {
        test_horner_poly_eval!(f32);
    }
}
