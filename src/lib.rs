pub mod errors;
pub mod m1d;
pub mod m2d;
pub mod measurement;
pub mod percentage;
pub mod prefix;
pub mod ranged_measurement;
pub mod uom;

// Prelude module
pub mod prelude {
    pub use super::m1d::M1d;
    pub use super::m2d::M2d;
    pub use super::measurement::*;
    pub use super::percentage::*;
    pub use super::prefix::*;
    pub use super::ranged_measurement::*;
    pub use super::uom::*;
}

#[cfg(test)]
mod lib_test {
    use ndarray::Array2;

    use crate::{m2d::M2d, prefix::Prefix, uom::Volt};

    #[test]
    fn get_values() {
        let m = M2d::<Volt>::new(
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap(),
            Prefix::Milli,
        );
        let _e = m.clone() + m.clone();
        assert_eq!(
            m.values(),
            Array2::from_shape_vec((2, 2), vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap()
        );
    }
}
