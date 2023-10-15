#[macro_export]
macro_rules! refine_iterative {
    ( $order:literal, $t:tt, $tol:expr, $max_iter:literal ) => {
        |ecc, mean_anom, ecc_anom| {
            let mut result = ecc_anom;
            for _ in 0..$max_iter {
                let (next, diff) = kepler_householder_step!($order, $t)(ecc, mean_anom, result);
                result = result + next;
                if diff.abs() < $tol {
                    break;
                }
            }
            result
        }
    };
}

#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;
    use libkepler_householder::kepler_householder_step;

    #[test]
    fn test_iterative() {
        let ecc = 0.65f64;
        let expected = 0.456f64;
        let mean_anom = expected - ecc * expected.sin();
        let actual = refine_iterative!(1, f64, 1e-8, 100)(ecc, mean_anom, expected);
        assert_abs_diff_eq!(actual, expected, epsilon = 1e-8);
    }
}
