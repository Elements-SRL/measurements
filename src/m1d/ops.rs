use std::ops::{Add, Div, Mul, Sub};

use crate::{
    errors::MeasurementErrors,
    m1d::M1d,
    uom::{Adimensional, Uom},
};

impl<U: Uom> Add<f64> for M1d<U> {
    type Output = M1d<U>;
    fn add(self, rhs: f64) -> Self::Output {
        let values = self.values + rhs;
        Self { values, ..self }
    }
}

impl<U: Uom> Sub<f64> for M1d<U> {
    type Output = M1d<U>;
    fn sub(self, rhs: f64) -> Self::Output {
        let values = self.values - rhs;
        Self { values, ..self }
    }
}

impl<U: Uom> Mul<f64> for M1d<U> {
    type Output = M1d<U>;
    fn mul(self, rhs: f64) -> Self::Output {
        let values = self.values * rhs;
        Self { values, ..self }
    }
}

impl<U: Uom> Div<f64> for M1d<U> {
    type Output = M1d<U>;
    fn div(self, rhs: f64) -> Self::Output {
        let values = self.values / rhs;
        Self { values, ..self }
    }
}

impl<U: Uom> Add for M1d<U> {
    type Output = Result<M1d<U>, MeasurementErrors>;
    fn add(self, rhs: M1d<U>) -> Self::Output {
        let b1 = self.values();
        let b2 = rhs.values();
        let s1 = b1.shape();
        let s2 = b2.shape();
        match (s1, s2) {
            (x, y) if x == y => {
                let n = rhs.clone().convert_to(self.prefix());
                let nv = n.values() + self.values();
                Ok(Self { values: nv, ..self })
            }
            _ => Err(MeasurementErrors::DifferentShape(s1.to_vec(), s2.to_vec())),
        }
    }
}

impl<U: Uom> Sub for M1d<U> {
    type Output = Result<M1d<U>, MeasurementErrors>;
    fn sub(self, rhs: M1d<U>) -> Self::Output {
        let b1 = self.values();
        let b2 = rhs.values();
        let s1 = b1.shape();
        let s2 = b2.shape();
        match (s1, s2) {
            (x, y) if x == y => {
                let n = rhs.convert_to(self.prefix());
                let nv = self.values() - n.values();
                Ok(Self { values: nv, ..self })
            }
            _ => Err(MeasurementErrors::DifferentShape(s1.to_vec(), s2.to_vec())),
        }
    }
}

impl<U: Uom> Div for M1d<U> {
    type Output = Result<M1d<Adimensional>, MeasurementErrors>;
    fn div(self, rhs: M1d<U>) -> Self::Output {
        let b1 = self.values();
        let b2 = rhs.values();
        let s1 = b1.shape();
        let s2 = b2.shape();
        match (s1, s2) {
            (x, y) if x == y => {
                let n = rhs.convert_to(self.prefix());
                let nv = self.values() / n.values();
                Ok(M1d::new(nv, self.prefix()))
            }
            _ => Err(MeasurementErrors::DifferentShape(s1.to_vec(), s2.to_vec())),
        }
    }
}

#[cfg(test)]
mod m1d_ops_tests {
    use crate::{m1d::M1d, prefix::Prefix, uom::Volt};

    #[test]
    fn sum_m1d_m1d() {
        let m1 = M1d::<Volt>::new(vec![1000.0, 2000.0, 3000.0, 4000.0], Prefix::Milli);

        let m2 = M1d::new(vec![1.0, 2.0, 3.0, 4.0], Prefix::None);
        let ctrl = M1d::new(vec![2000.0, 4000.0, 6000.0, 8000.0], Prefix::Milli);
        assert_eq!((m1 + m2).unwrap(), ctrl);
    }

    #[test]
    fn sum_m1d_scalar() {
        let m1 = M1d::<Volt>::new(vec![1000.0, 2000.0, 3000.0, 4000.0], Prefix::Milli);

        let ctrl = M1d::new(vec![2000.0, 3000.0, 4000.0, 5000.0], Prefix::Milli);
        assert_eq!(m1 + 1000.0, ctrl);
    }

    #[test]
    fn sum_m1d_m1d_wrong_dims() {
        let m1 = M1d::<Volt>::new(vec![2000.0, 3000.0, 4000.0, 5000.0], Prefix::Milli);
        let m2 = M1d::<Volt>::new(vec![2000.0, 3000.0, 4000.0], Prefix::Milli);
        assert!((m1 + m2).is_err());
    }

    #[test]
    fn sub_m1d_m1d() {
        let m1 = M1d::<Volt>::new(vec![1000.0, 2000.0, 3000.0, 4000.0], Prefix::Milli);

        let ctrl = M1d::new(vec![0.0, 0.0, 0.0, 0.0], Prefix::Milli);
        assert_eq!((m1.clone() - m1).unwrap(), ctrl);
    }

    #[test]
    fn sub_m1d_scalar() {
        let m1 = M1d::<Volt>::new(vec![1000.0, 2000.0, 3000.0, 4000.0], Prefix::Milli);

        let ctrl = M1d::new(vec![0.0, 1000.0, 2000.0, 3000.0], Prefix::Milli);
        assert_eq!(m1 - 1000.0, ctrl);
    }

    #[test]
    fn sub_m1d_m1d_wrong_dims() {
        let m1 = M1d::<Volt>::new(vec![4000.0, 3000.0, 2000.0], Prefix::Milli);
        let m2 = M1d::<Volt>::new(vec![4000.0, 3000.0, 2000.0, 1000.0], Prefix::Milli);
        assert!((m1 - m2).is_err());
    }

    #[test]
    fn duv_m1d_m1d() {
        let m1 = M1d::<Volt>::new(vec![1000.0, 2000.0, 3000.0, 4000.0], Prefix::Milli);

        let ctrl = M1d::new(vec![1.0, 1.0, 1.0, 1.0], Prefix::Milli);
        assert_eq!((m1.clone() / m1).unwrap(), ctrl);
    }

    #[test]
    fn div_m1d_scalar() {
        let m1 = M1d::<Volt>::new(vec![1000.0, 2000.0, 3000.0, 4000.0], Prefix::Milli);

        let ctrl = M1d::new(vec![1.0, 2.0, 3.0, 4.0], Prefix::Milli);
        assert_eq!(m1 / 1000.0, ctrl);
    }

    #[test]
    fn div_m1d_m1d_wrong_dims() {
        let m1 = M1d::<Volt>::new(vec![4000.0, 3000.0, 2000.0], Prefix::Milli);
        let m2 = M1d::<Volt>::new(vec![4000.0, 3000.0, 2000.0, 1000.0], Prefix::Milli);
        assert!((m1 / m2).is_err());
    }
}
