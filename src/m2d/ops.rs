use std::ops::{Add, Div, Mul, Sub};

use crate::{
    m2d::{M2d, errors::ErrorCodes},
    uom::{Adimensional, Uom},
};

impl<U: Uom> Add<f64> for M2d<U> {
    type Output = M2d<U>;
    fn add(self, rhs: f64) -> Self::Output {
        let values = self.values + rhs;
        Self { values, ..self }
    }
}

impl<U: Uom> Sub<f64> for M2d<U> {
    type Output = M2d<U>;
    fn sub(self, rhs: f64) -> Self::Output {
        let values = self.values - rhs;
        Self { values, ..self }
    }
}

impl<U: Uom> Mul<f64> for M2d<U> {
    type Output = M2d<U>;
    fn mul(self, rhs: f64) -> Self::Output {
        let values = self.values * rhs;
        Self { values, ..self }
    }
}

impl<U: Uom> Div<f64> for M2d<U> {
    type Output = M2d<U>;
    fn div(self, rhs: f64) -> Self::Output {
        let values = self.values / rhs;
        Self { values, ..self }
    }
}

impl<U: Uom> Add for M2d<U> {
    type Output = Result<M2d<U>, ErrorCodes>;
    fn add(self, rhs: M2d<U>) -> Self::Output {
        let b1 = self.values();
        let b2 = rhs.values();
        let s1 = b1.shape();
        let s2 = b2.shape();
        match (s1, s2) {
            (x, y) if x == y => {
                let n = rhs.convert_to(self.prefix());
                let nv = n.values() + self.values();
                Ok(Self { values: nv, ..self })
            }
            _ => Err(ErrorCodes::DifferentShape(s1.to_vec(), s2.to_vec())),
        }
    }
}

impl<U: Uom> Sub for M2d<U> {
    type Output = Result<M2d<U>, ErrorCodes>;
    fn sub(self, rhs: M2d<U>) -> Self::Output {
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
            _ => Err(ErrorCodes::DifferentShape(s1.to_vec(), s2.to_vec())),
        }
    }
}

impl<U: Uom> Div for M2d<U> {
    type Output = Result<M2d<Adimensional>, ErrorCodes>;
    fn div(self, rhs: M2d<U>) -> Self::Output {
        let b1 = self.values();
        let b2 = rhs.values();
        let s1 = b1.shape();
        let s2 = b2.shape();
        match (s1, s2) {
            (x, y) if x == y => {
                let n = rhs.convert_to(self.prefix());
                let nv = self.values() / n.values();
                Ok(M2d::new(nv, self.prefix()))
            }
            _ => Err(ErrorCodes::DifferentShape(s1.to_vec(), s2.to_vec())),
        }
    }
}

#[cfg(test)]
mod m2d_ops_tests {
    use ndarray::Array2;

    use crate::{m2d::M2d, prefix::Prefix, uom::Volt};

    #[test]
    fn sum_m2d_m2d() {
        let m1 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(),
            Prefix::Milli,
        );

        let m2 = M2d::new(
            Array2::from_shape_vec((2, 2), vec![4000.0, 3000.0, 2000.0, 1000.0]).unwrap(),
            Prefix::Milli,
        );
        let ctrl = M2d::new(
            Array2::from_shape_vec((2, 2), vec![5000.0, 5000.0, 5000.0, 5000.0]).unwrap(),
            Prefix::Milli,
        );
        assert_eq!((m1 + m2).unwrap(), ctrl);
    }

    #[test]
    fn sum_m2d_scalar() {
        let m1 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(),
            Prefix::Milli,
        );

        let ctrl = M2d::new(
            Array2::from_shape_vec((2, 2), vec![2000.0, 3000.0, 4000.0, 5000.0]).unwrap(),
            Prefix::Milli,
        );
        assert_eq!(m1 + 1000.0, ctrl);
    }

    #[test]
    fn sum_m2d_m2d_wrong_dims() {
        let m1 = M2d::<Volt>::new(
            Array2::from_shape_fn((3, 2), |(i, j)| (1.0 + i as f64) * (1.0 + j as f64)),
            Prefix::Milli,
        );
        let m2 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![4000.0, 3000.0, 2000.0, 1000.0]).unwrap(),
            Prefix::Milli,
        );
        assert!((m1 + m2).is_err());
    }

    #[test]
    fn sub_m2d_m2d() {
        let m1 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(),
            Prefix::Milli,
        );

        let ctrl = M2d::new(
            Array2::from_shape_vec((2, 2), vec![0.0, 0.0, 0.0, 0.0]).unwrap(),
            Prefix::Milli,
        );
        assert_eq!((m1.clone() - m1).unwrap(), ctrl);
    }

    #[test]
    fn sub_m2d_scalar() {
        let m1 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(),
            Prefix::Milli,
        );

        let ctrl = M2d::new(
            Array2::from_shape_vec((2, 2), vec![0.0, 1000.0, 2000.0, 3000.0]).unwrap(),
            Prefix::Milli,
        );
        assert_eq!(m1 - 1000.0, ctrl);
    }

    #[test]
    fn sub_m2d_m2d_wrong_dims() {
        let m1 = M2d::<Volt>::new(
            Array2::from_shape_fn((3, 2), |(i, j)| (1.0 + i as f64) * (1.0 + j as f64)),
            Prefix::Milli,
        );
        let m2 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![4000.0, 3000.0, 2000.0, 1000.0]).unwrap(),
            Prefix::Milli,
        );
        assert!((m1 - m2).is_err());
    }

    #[test]
    fn duv_m2d_m2d() {
        let m1 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(),
            Prefix::Milli,
        );

        let ctrl = M2d::new(
            Array2::from_shape_vec((2, 2), vec![1.0, 1.0, 1.0, 1.0]).unwrap(),
            Prefix::Milli,
        );
        assert_eq!((m1.clone() / m1).unwrap(), ctrl);
    }

    #[test]
    fn div_m2d_scalar() {
        let m1 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(),
            Prefix::Milli,
        );

        let ctrl = M2d::new(
            Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(),
            Prefix::Milli,
        );
        assert_eq!(m1 / 1000.0, ctrl);
    }

    #[test]
    fn div_m2d_m2d_wrong_dims() {
        let m1 = M2d::<Volt>::new(
            Array2::from_shape_fn((3, 2), |(i, j)| (1.0 + i as f64) * (1.0 + j as f64)),
            Prefix::Milli,
        );
        let m2 = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![4000.0, 3000.0, 2000.0, 1000.0]).unwrap(),
            Prefix::Milli,
        );
        assert!((m1 / m2).is_err());
    }
}
