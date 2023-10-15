use approx::assert_abs_diff_eq;
use libkepler_householder::kepler_householder_step;
use num_traits::{AsPrimitive, Float};

macro_rules! test_algorithm {
    ( $t:tt, $order:literal, $algo:expr ) => {
        let size = 100usize;
        let ecc = 0.65 as $t;
        let mean_anom = 0.123 as $t;
        for n in 0..size {
            let ecc_anom = std::$t::consts::PI * (n as $t) / ((size - 1) as $t);
            let expected = $algo(ecc, mean_anom, ecc_anom);
            let actual = kepler_householder_step!($order, $t)(ecc, mean_anom, ecc_anom);
            assert_abs_diff_eq!(actual, expected, epsilon = 10 as $t * $t::EPSILON);
        }
    };
}

fn newton_reference<T: Float>(e: T, mean_anom: T, ecc_anom: T) -> T {
    let fi = ecc_anom - e * ecc_anom.sin() - mean_anom;
    let fip = T::one() - e * ecc_anom.cos();
    return -fi / fip;
}

#[test]
fn test_compare_newton() {
    test_algorithm!(f32, 1, newton_reference);
    test_algorithm!(f64, 1, newton_reference);
}

fn halley_reference<T: Float + 'static>(e: T, mean_anom: T, ecc_anom: T) -> T
where
    f64: AsPrimitive<T>,
{
    let fi = ecc_anom - e * ecc_anom.sin() - mean_anom;
    let fip = T::one() - e * ecc_anom.cos();
    let fipp = e * ecc_anom.sin();
    let fippp = T::one() - fip;
    let d1 = -fi / fip;
    let d1 = -fi / (fip + d1 * fipp / 2.0.as_());
    let d1 = -fi / (fip + d1 * fipp / 2.0.as_() + d1 * d1 * fippp / 6.0.as_());
    d1
}

#[test]
fn test_compare_halley() {
    test_algorithm!(f32, 3, halley_reference);
    test_algorithm!(f64, 3, halley_reference);
}

fn fourth_reference<T: Float + 'static>(e: T, mean_anom: T, ecc_anom: T) -> T
where
    f64: AsPrimitive<T>,
{
    let fi = ecc_anom - e * ecc_anom.sin() - mean_anom;
    let fip = T::one() - e * ecc_anom.cos();
    let fipp = e * ecc_anom.sin();
    let fippp = T::one() - fip;
    let fipppp = -fipp;
    let d1 = -fi / fip;
    let d1 = -fi / (fip + d1 * fipp / 2.0.as_());
    let d1 = -fi / (fip + d1 * fipp / 2.0.as_() + d1 * d1 * fippp / 6.0.as_());
    let d1 = -fi
        / (fip
            + d1 * fipp / 2.0.as_()
            + d1 * d1 * fippp / 6.0.as_()
            + d1 * d1 * d1 * fipppp / 24.0.as_());
    d1
}

#[test]
fn test_compare_fourth() {
    test_algorithm!(f32, 4, fourth_reference);
    test_algorithm!(f64, 4, fourth_reference);
}
