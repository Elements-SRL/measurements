use crate::{prefix::Prefix, uom::Uom};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct M2d<U: Uom> {
    values: Array2<f64>,
    prefix: Prefix,
    uom: PhantomData<U>,
}

impl<U: Uom> M2d<U> {
    pub fn new<T: Into<Array2<f64>>>(values: T, prefix: Prefix) -> Self {
        Self {
            values: values.into(),
            prefix,
            uom: PhantomData,
        }
    }

    pub fn values(&self) -> Array2<f64> {
        self.values.clone()
    }

    pub fn prefix(&self) -> Prefix {
        self.prefix
    }

    fn convert_to(self, pfx: Prefix) -> Self {
        let conversion_factor = self.prefix.get_conversion_factor(pfx);
        if conversion_factor == 1.0 {
            self.clone()
        } else {
            let mut s = self;
            s.values.par_mapv_inplace(|x| x * conversion_factor);
            Self {
                values: s.values,
                prefix: pfx,
                uom: PhantomData,
            }
        }
    }
}

impl<U: Uom> PartialEq for M2d<U> {
    fn eq(&self, other: &Self) -> bool {
        if self.prefix != other.prefix {
            self.clone().convert_to(other.prefix()).values == other.values
        } else {
            self.values == other.values
        }
    }
}

#[cfg(test)]
mod m2d_tests {
    use super::*;
    use crate::uom::Volt;

    #[test]
    fn get_values() {
        let m = M2d::<Volt>::new(Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(), Prefix::Milli);
        assert_eq!(m.values(), Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap());
    }

    #[test]
    fn convert_to() {
        let m = M2d::<Volt>::new( Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(), Prefix::Milli);
        assert_eq!(
            m.convert_to(Prefix::Micro).values(),
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap()
        );
    }

    #[test]
    fn convert_to_2() {
        let m1 = M2d::<Volt>::new( Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(), Prefix::Milli);
        let m2 = M2d::<Volt>::new( Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(), Prefix::Micro);
        assert_eq!(m1, m2);
    }

    #[test]
    fn convert_to_3() {
        // should short-circuit if prefixes are the same
        let m = M2d::<Volt>::new(Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap(), Prefix::Milli);
        let m2 = m.clone();
        assert_eq!(m, m2);
    }
}
