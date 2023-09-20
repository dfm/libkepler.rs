mod horner;
pub mod householder;
mod refiner;

// use num_traits::Float;

// #[derive(thiserror::Error, Debug)]
// pub enum KeplerError<F: Float> {
//     #[error("setup must be called first")]
//     SetupRequired,

//     #[error("solver did not converge")]
//     DidNotConverge((F, F)),
// }

// fn starter_simple<F: Float>(mean_anom: F, eccen: F) -> F {
//     mean_anom + mean_anom.sin().signum() * 0.85 * eccen // sign(sin(M)) * 0.85 * e;
// }

// trait KeplerSolver {
//     fn setup(&mut self, eccen: Float);
//     fn solve(&self, mean_anom: Float) -> Option<(Float, Float)>;
// }

// pub struct FirstOrderIterative<const MAX_ITER: usize> {
//     tol: Float,
//     eccen: Option<Float>,
// }

// impl<const MAX_ITER: usize> FirstOrderIterative<MAX_ITER> {
//     pub fn new(tol: Float) -> Self {
//         FirstOrderIterative { tol, eccen: None }
//     }
// }

// impl<const MAX_ITER: usize> KeplerSolver for FirstOrderIterative<MAX_ITER> {
//     fn setup(&mut self, eccen: Float) {
//         self.eccen = Some(eccen);
//     }

//     fn solve(&self, mean_anom: Float) -> Option<(Float, Float)> {
//         let eccen = self.eccen?;
//         let mut ecc_anom = starter_simple(mean_anom, eccen);
//         let mut sin_ecc_anom = ecc_anom.sin();
//         let mut cos_ecc_anom = ecc_anom.cos();
//         for _ in 0..MAX_ITER {
//             let f = ecc_anom - eccen * sin_ecc_anom - mean_anom;
//             if f.abs() < self.tol {
//                 return Some((sin_ecc_anom, cos_ecc_anom));
//             }
//             let f_prime = 1.0 - eccen * cos_ecc_anom;
//             ecc_anom -= f / f_prime;
//             sin_ecc_anom = ecc_anom.sin();
//             cos_ecc_anom = ecc_anom.cos();
//         }
//         None
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
