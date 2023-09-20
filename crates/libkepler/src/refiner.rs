use crate::householder::HouseholderModel;
use num_traits::Float;

pub(crate) struct Evaluator<T> {
    pub(crate) f0: T,
    ecc_sin: T,
    ecc_cos: T,
}

impl<T: Copy + Float> Evaluator<T> {
    pub(crate) fn new(ecc: T, mean_anom: T, ecc_anom: T) -> Self {
        let ecc_sin = ecc * ecc_anom.sin();
        let ecc_cos = ecc * ecc_anom.cos();
        let f0 = ecc_anom - ecc_sin - mean_anom;
        Evaluator {
            f0,
            ecc_sin,
            ecc_cos,
        }
    }
}

impl<T: Copy + Float> HouseholderModel<T> for Evaluator<T> {
    fn evaluate<const ORDER: usize>(&self) -> T {
        match ORDER {
            0 => self.f0,
            1 => T::one() - self.ecc_cos,
            2 => self.ecc_sin,
            3 => self.ecc_cos,
            _ => {
                let sign = if ORDER % 4 < 2 { -T::one() } else { T::one() };
                let value = if ORDER % 2 == 0 {
                    self.ecc_sin
                } else {
                    self.ecc_cos
                };
                sign * value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_evaluator() {
        let ecc: f64 = 0.65;
        let mean_anom: f64 = 0.123;
        let ecc_anom: f64 = 0.456;
        let evaluator = Evaluator::new(ecc, mean_anom, ecc_anom);
        assert_abs_diff_eq!(
            evaluator.evaluate::<0>(),
            ecc_anom - ecc * ecc_anom.sin() - mean_anom
        );
        assert_abs_diff_eq!(evaluator.evaluate::<1>(), 1.0 - ecc * ecc_anom.cos());
        assert_abs_diff_eq!(evaluator.evaluate::<2>(), ecc * ecc_anom.sin());
        assert_abs_diff_eq!(evaluator.evaluate::<3>(), ecc * ecc_anom.cos());
        assert_abs_diff_eq!(evaluator.evaluate::<4>(), -ecc * ecc_anom.sin());
        assert_abs_diff_eq!(evaluator.evaluate::<5>(), -ecc * ecc_anom.cos());
        assert_abs_diff_eq!(evaluator.evaluate::<6>(), ecc * ecc_anom.sin());
        assert_abs_diff_eq!(evaluator.evaluate::<7>(), ecc * ecc_anom.cos());
        assert_abs_diff_eq!(evaluator.evaluate::<8>(), -ecc * ecc_anom.sin());
        assert_abs_diff_eq!(evaluator.evaluate::<9>(), -ecc * ecc_anom.cos());
    }
}
