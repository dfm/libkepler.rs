use num_traits::{AsPrimitive, Float};

pub trait Starter<T> {
    fn new(ecc: T) -> Self;
    fn start(&self, mean_anom: T) -> T;
}

pub struct NoOpStarter {}

impl<T> Starter<T> for NoOpStarter {
    fn new(_: T) -> Self {
        NoOpStarter {}
    }
    fn start(&self, mean_anom: T) -> T {
        mean_anom
    }
}

pub struct SimpleStarter<T> {
    ecc: T,
}

impl<T: Float + 'static> Starter<T> for SimpleStarter<T>
where
    f64: AsPrimitive<T>,
{
    fn new(ecc: T) -> Self {
        SimpleStarter { ecc }
    }
    fn start(&self, mean_anom: T) -> T {
        mean_anom * 0.85.as_() * self.ecc
    }
}

pub struct MikkolaStarter<T> {
    ecc: T,
    alpha: T,
    alpha3: T,
}

impl<T: Float + 'static> Starter<T> for MikkolaStarter<T>
where
    f64: AsPrimitive<T>,
{
    fn new(ecc: T) -> Self {
        let factor = T::one() / (4.0.as_() * ecc + 0.5.as_());
        let alpha = (T::one() - ecc) * factor;
        MikkolaStarter {
            ecc,
            alpha,
            alpha3: alpha * alpha * alpha,
        }
    }
    fn start(&self, mean_anom: T) -> T {
        let beta = 0.5.as_() * mean_anom * self.ecc;
        let arg: T = beta.copysign((beta * beta + self.alpha3).sqrt());
        let z = (beta + arg).cbrt();
        let s = z - self.alpha / z;
        let s = s - 0.078.as_() * s.powi(5) / (1.0.as_() + self.ecc);
        mean_anom + self.ecc * s * (3.0.as_() - 4.0.as_() * s * s)
    }
}
